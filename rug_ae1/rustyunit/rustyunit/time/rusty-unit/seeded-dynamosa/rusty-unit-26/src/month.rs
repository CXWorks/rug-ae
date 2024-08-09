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
#[timeout(30000)]fn rusty_test_3209() {
//    rusty_monitor::set_test_id(3209);
    let mut i32_0: i32 = -83i32;
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 23u8;
    let mut u8_2: u8 = 28u8;
    let mut i32_1: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut u32_1: u32 = 1000000000u32;
    let mut u8_3: u8 = 59u8;
    let mut u8_4: u8 = 7u8;
    let mut u8_5: u8 = 12u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_2: i32 = 187i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut i64_0: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_2: u32 = 10000000u32;
    let mut u8_6: u8 = 3u8;
    let mut u8_7: u8 = 94u8;
    let mut u8_8: u8 = 81u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_1: i64 = 154i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_3: i32 = 195i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_1, duration_1);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_2);
    let mut tuple_0: (u8, u8, u8, u16) = crate::time::Time::as_hms_milli(time_2);
    let mut month_1: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut month_2: month::Month = crate::month::Month::June;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_1_ref_0, month_0_ref_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_0, i32_0);
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(month_2_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6557() {
//    rusty_monitor::set_test_id(6557);
    let mut i32_0: i32 = 4i32;
    let mut i64_0: i64 = -68i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 77u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 86399i32;
    let mut i64_1: i64 = 111i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut u32_1: u32 = 10000u32;
    let mut u8_3: u8 = 31u8;
    let mut u8_4: u8 = 52u8;
    let mut u8_5: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut u8_6: u8 = 2u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut month_1: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1687() {
//    rusty_monitor::set_test_id(1687);
    let mut i64_0: i64 = 3600i64;
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 86399i32;
    let mut i64_1: i64 = 111i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i32_1: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut u8_0: u8 = 10u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_0);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_3870() {
//    rusty_monitor::set_test_id(3870);
    let mut i64_0: i64 = -68i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 77u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u8_3: u8 = 10u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_3);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8460() {
//    rusty_monitor::set_test_id(8460);
    let mut i64_0: i64 = -68i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 10000u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut u8_3: u8 = 10u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_3);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_2035() {
//    rusty_monitor::set_test_id(2035);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut i32_0: i32 = 342i32;
    let mut i64_0: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i32_1: i32 = 88i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut i32_2: i32 = 195i32;
    let mut i64_1: i64 = 86400i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 12u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_3: i32 = 187i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut i64_2: i64 = 86400i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut u32_1: u32 = 10000000u32;
    let mut u8_3: u8 = 3u8;
    let mut u8_4: u8 = 94u8;
    let mut u8_5: u8 = 81u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_3: i64 = 154i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut month_1: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut month_2: month::Month = crate::month::Month::June;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_1_ref_0, month_0_ref_0);
    let mut i32_4: i32 = crate::date::Date::year(date_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_969() {
//    rusty_monitor::set_test_id(969);
    let mut i8_0: i8 = 4i8;
    let mut i8_1: i8 = 127i8;
    let mut i8_2: i8 = 65i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 291i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut i32_1: i32 = 128i32;
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_2: i32 = 172i32;
    let mut i64_1: i64 = 60i64;
    let mut i64_2: i64 = 0i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_4_ref_0: &crate::duration::Duration = &mut duration_4;
    let mut i8_3: i8 = 4i8;
    let mut i64_3: i64 = 1000i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_sub_std(time_1, duration_2);
    let mut tuple_1: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_1);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = std::clone::Clone::clone(month_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7857() {
//    rusty_monitor::set_test_id(7857);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 77u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 86399i32;
    let mut i64_0: i64 = 111i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut u32_1: u32 = 10000u32;
    let mut u8_3: u8 = 31u8;
    let mut u8_4: u8 = 52u8;
    let mut u8_5: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut u8_6: u8 = 10u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut month_1: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_134() {
//    rusty_monitor::set_test_id(134);
    let mut i64_0: i64 = 2147483647i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = 24i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut u16_0: u16 = 59u16;
    let mut i64_2: i64 = 253402300799i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 24i8;
    let mut i8_4: i8 = 4i8;
    let mut i8_5: i8 = 50i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 12u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 100000000u32;
    let mut u8_3: u8 = 60u8;
    let mut u8_4: u8 = 0u8;
    let mut u8_5: u8 = 23u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_1: u16 = 59u16;
    let mut i32_0: i32 = 48i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_1};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut u16_2: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_157() {
//    rusty_monitor::set_test_id(157);
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 7u8;
    let mut u8_3: u8 = 1u8;
    let mut u8_4: u8 = 59u8;
    let mut u8_5: u8 = 8u8;
    let mut u8_6: u8 = 30u8;
    let mut u8_7: u8 = 3u8;
    let mut u8_8: u8 = 52u8;
    let mut u8_9: u8 = 2u8;
    let mut u8_10: u8 = 98u8;
    let mut u8_11: u8 = 31u8;
    let mut u8_12: u8 = 23u8;
    let mut u8_13: u8 = 30u8;
    let mut u8_14: u8 = 43u8;
    let mut u8_15: u8 = 89u8;
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
#[timeout(30000)]fn rusty_test_6756() {
//    rusty_monitor::set_test_id(6756);
    let mut i64_0: i64 = -68i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 86399i32;
    let mut i64_1: i64 = 111i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut u32_0: u32 = 10000u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 52u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut u8_3: u8 = 10u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_3);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1472() {
//    rusty_monitor::set_test_id(1472);
    let mut i8_0: i8 = 127i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = 4828193600913801216.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_0: i64 = 1000000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_0: i32 = 93i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut i32_1: i32 = 150i32;
    let mut i64_1: i64 = -68i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 77u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_3: i8 = 5i8;
    let mut i8_4: i8 = 24i8;
    let mut i8_5: i8 = 60i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_2: i32 = 86399i32;
    let mut i64_2: i64 = 111i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut u8_3: u8 = 10u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_3);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut u8_4: u8 = crate::offset_date_time::OffsetDateTime::second(offsetdatetime_1);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_668() {
//    rusty_monitor::set_test_id(668);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut month_2: month::Month = crate::month::Month::September;
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut month_3: month::Month = crate::month::Month::June;
    let mut month_3_ref_0: &month::Month = &mut month_3;
    let mut month_4: month::Month = crate::month::Month::June;
    let mut month_4_ref_0: &month::Month = &mut month_4;
    let mut month_5: month::Month = crate::month::Month::September;
    let mut month_5_ref_0: &month::Month = &mut month_5;
    let mut i32_0: i32 = 65i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut month_6: month::Month = crate::date::Date::month(date_0);
    let mut month_6_ref_0: &month::Month = &mut month_6;
    let mut month_7: month::Month = crate::month::Month::May;
    let mut month_7_ref_0: &month::Month = &mut month_7;
    let mut month_8: month::Month = crate::month::Month::March;
    let mut month_8_ref_0: &month::Month = &mut month_8;
    let mut month_9: month::Month = crate::month::Month::March;
    let mut month_9_ref_0: &month::Month = &mut month_9;
    let mut month_10: month::Month = std::clone::Clone::clone(month_9_ref_0);
    let mut month_11: month::Month = std::clone::Clone::clone(month_8_ref_0);
    let mut month_12: month::Month = std::clone::Clone::clone(month_7_ref_0);
    let mut month_13: month::Month = std::clone::Clone::clone(month_6_ref_0);
    let mut month_14: month::Month = std::clone::Clone::clone(month_5_ref_0);
    let mut month_15: month::Month = std::clone::Clone::clone(month_4_ref_0);
    let mut month_16: month::Month = std::clone::Clone::clone(month_3_ref_0);
    let mut month_17: month::Month = std::clone::Clone::clone(month_2_ref_0);
    let mut month_18: month::Month = std::clone::Clone::clone(month_1_ref_0);
    let mut month_19: month::Month = std::clone::Clone::clone(month_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_7025() {
//    rusty_monitor::set_test_id(7025);
    let mut u32_0: u32 = 71u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 30u8;
    let mut i64_0: i64 = -125i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i32_0: i32 = 139i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut i64_1: i64 = -68i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u32_1: u32 = 0u32;
    let mut u8_3: u8 = 24u8;
    let mut u8_4: u8 = 77u8;
    let mut u8_5: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 86399i32;
    let mut i64_2: i64 = 111i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut u32_2: u32 = 10000u32;
    let mut u8_6: u8 = 31u8;
    let mut u8_7: u8 = 52u8;
    let mut u8_8: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_2: i32 = 156i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut u8_9: u8 = 10u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_9);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut month_1: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_1, u8_2, u8_1, u8_0, u32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_600() {
//    rusty_monitor::set_test_id(600);
    let mut u8_0: u8 = 29u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 59u8;
    let mut u8_3: u8 = 12u8;
    let mut u8_4: u8 = 29u8;
    let mut u8_5: u8 = 23u8;
    let mut u8_6: u8 = 6u8;
    let mut u8_7: u8 = 16u8;
    let mut u8_8: u8 = 3u8;
    let mut u8_9: u8 = 30u8;
    let mut u8_10: u8 = 6u8;
    let mut u8_11: u8 = 79u8;
    let mut u8_12: u8 = 25u8;
    let mut u8_13: u8 = 12u8;
    let mut u8_14: u8 = 1u8;
    let mut u8_15: u8 = 1u8;
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
#[timeout(30000)]fn rusty_test_8056() {
//    rusty_monitor::set_test_id(8056);
    let mut i32_0: i32 = -34i32;
    let mut i64_0: i64 = -68i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut u32_0: u32 = 2u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 77u8;
    let mut u8_2: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 86399i32;
    let mut i64_1: i64 = 111i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut u32_1: u32 = 10000u32;
    let mut u8_3: u8 = 31u8;
    let mut u8_4: u8 = 52u8;
    let mut u8_5: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut u8_6: u8 = 10u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut month_1: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9093() {
//    rusty_monitor::set_test_id(9093);
    let mut u32_0: u32 = 81u32;
    let mut u8_0: u8 = 0u8;
    let mut u8_1: u8 = 11u8;
    let mut u8_2: u8 = 3u8;
    let mut i32_0: i32 = 32i32;
    let mut i64_0: i64 = -12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_1: i64 = 2440588i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut duration_2_ref_0: &crate::duration::Duration = &mut duration_2;
    let mut i64_2: i64 = -68i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut u32_1: u32 = 0u32;
    let mut u8_3: u8 = 24u8;
    let mut u8_4: u8 = 77u8;
    let mut u8_5: u8 = 7u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 86399i32;
    let mut i64_3: i64 = 111i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_1);
    let mut u32_2: u32 = 10000u32;
    let mut u8_6: u8 = 31u8;
    let mut u8_7: u8 = 52u8;
    let mut u8_8: u8 = 52u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_2: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut u8_9: u8 = 10u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_9);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut month_1: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut duration_5_ref_0: &crate::duration::Duration = &mut duration_5;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut month_1_ref_0: &month::Month = &mut month_1;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4723() {
//    rusty_monitor::set_test_id(4723);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut u32_0: u32 = 1000000000u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 12u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 187i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut i64_0: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_1: i64 = 154i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_1: i32 = 195i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut month_1: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut month_2: month::Month = crate::month::Month::June;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_1_ref_0, month_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_99() {
//    rusty_monitor::set_test_id(99);
    let mut u16_0: u16 = 365u16;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 10u8;
    let mut u8_2: u8 = 3u8;
    let mut u16_1: u16 = 366u16;
    let mut i32_0: i32 = 325i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_1);
    let mut u32_0: u32 = 61u32;
    let mut u8_3: u8 = 31u8;
    let mut u8_4: u8 = 29u8;
    let mut u8_5: u8 = 24u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = 31i8;
    let mut i8_2: i8 = 48i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = 24i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut u8_6: u8 = 6u8;
    let mut i64_2: i64 = 1i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut month_1: month::Month = crate::month::Month::December;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_1_ref_0, month_0_ref_0);
    let mut i16_0: i16 = crate::utc_offset::UtcOffset::whole_minutes(utcoffset_0);
    let mut u16_2: u16 = crate::time::Time::millisecond(time_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_2, u8_1, u8_0, u16_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5230() {
//    rusty_monitor::set_test_id(5230);
    let mut i32_0: i32 = -83i32;
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 23u8;
    let mut u8_2: u8 = 28u8;
    let mut i32_1: i32 = 156i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut month_0: month::Month = crate::month::Month::March;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut i32_2: i32 = 187i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_0: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_1: i64 = 154i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i32_3: i32 = 195i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut tuple_0: (u8, u8, u8, u16) = crate::time::Time::as_hms_milli(time_0);
    let mut month_1: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_0);
    let mut month_2: month::Month = crate::month::Month::June;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_1_ref_0, month_0_ref_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_0, i32_0);
//    panic!("From RustyUnit with love");
}
}