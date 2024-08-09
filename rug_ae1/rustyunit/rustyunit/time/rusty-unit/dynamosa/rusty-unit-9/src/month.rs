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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1886() {
    rusty_monitor::set_test_id(1886);
    let mut u16_0: u16 = 35u16;
    let mut i32_0: i32 = -10i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i8_0: i8 = 126i8;
    let mut i8_1: i8 = -26i8;
    let mut i8_2: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 32u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 59u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 79i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_1: u16 = 75u16;
    let mut i32_1: i32 = -21i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut u32_1: u32 = 44u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_6: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_2);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut i32_2: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_1);
    let mut month_3: month::Month = crate::month::Month::next(month_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8150() {
    rusty_monitor::set_test_id(8150);
    let mut i64_0: i64 = -18i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut i64_1: i64 = 56i64;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut i8_0: i8 = 65i8;
    let mut i8_1: i8 = -30i8;
    let mut i8_2: i8 = -12i8;
    let mut u16_0: u16 = 35u16;
    let mut i32_0: i32 = -10i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i8_3: i8 = 126i8;
    let mut i8_4: i8 = -26i8;
    let mut i8_5: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i64_2: i64 = 79i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut u16_1: u16 = 75u16;
    let mut i32_1: i32 = -21i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_2);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_0: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_0);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut i32_2: i32 = crate::offset_date_time::OffsetDateTime::to_julian_day(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4799() {
    rusty_monitor::set_test_id(4799);
    let mut u32_0: u32 = 78u32;
    let mut u8_0: u8 = 22u8;
    let mut u8_1: u8 = 20u8;
    let mut u8_2: u8 = 65u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = -20i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i8_0: i8 = -22i8;
    let mut i8_1: i8 = -59i8;
    let mut i8_2: i8 = 5i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_0, time_0);
    let mut i64_1: i64 = -88i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i128_0: i128 = 93i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i32_0: i32 = -27i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_1);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut i8_3: i8 = 1i8;
    let mut u16_0: u16 = 35u16;
    let mut i32_1: i32 = -10i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_4);
    let mut i8_4: i8 = 126i8;
    let mut i8_5: i8 = 91i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_3, i8_4);
    let mut i64_2: i64 = 67i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut u32_1: u32 = 32u32;
    let mut u8_3: u8 = 31u8;
    let mut u8_4: u8 = 55u8;
    let mut u8_5: u8 = 59u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_3: i64 = 79i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut u16_1: u16 = 75u16;
    let mut i32_2: i32 = -21i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_sub(date_5, duration_4);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_6, time_3);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_5, duration_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_6, utcoffset_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_2);
    let mut u32_2: u32 = 44u32;
    let mut u8_6: u8 = 46u8;
    let mut u8_7: u8 = 31u8;
    let mut u8_8: u8 = 70u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_9: u8 = 2u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_9);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_1);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut i32_3: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_3);
    let mut u8_10: u8 = crate::primitive_date_time::PrimitiveDateTime::second(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7662() {
    rusty_monitor::set_test_id(7662);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = 58i8;
    let mut i8_1: i8 = 42i8;
    let mut i8_2: i8 = 57i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = -144.329214f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u16_0: u16 = 16u16;
    let mut i32_0: i32 = 81i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut u16_1: u16 = 52u16;
    let mut i32_1: i32 = 79i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut i64_0: i64 = -85i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u8_0: u8 = 1u8;
    let mut i8_3: i8 = -42i8;
    let mut i8_4: i8 = -4i8;
    let mut i8_5: i8 = 60i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_2: i32 = 6i32;
    let mut i64_1: i64 = -46i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_3: i32 = 52i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i8_6: i8 = 0i8;
    let mut i8_7: i8 = -40i8;
    let mut i8_8: i8 = -46i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i8_9: i8 = 124i8;
    let mut i8_10: i8 = -23i8;
    let mut i8_11: i8 = 62i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_0);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut month_2: month::Month = crate::month::Month::April;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4810() {
    rusty_monitor::set_test_id(4810);
    let mut i8_0: i8 = 52i8;
    let mut i8_1: i8 = -45i8;
    let mut i8_2: i8 = -70i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 2u16;
    let mut i32_0: i32 = 206i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut u16_1: u16 = 35u16;
    let mut i32_1: i32 = -10i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_3: i8 = 126i8;
    let mut i8_4: i8 = -26i8;
    let mut i8_5: i8 = 91i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 32u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 59u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 79i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_2: u16 = 75u16;
    let mut i32_2: i32 = -21i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_1, time_0);
    let mut u32_1: u32 = 44u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_6: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_2);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut i32_3: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_2);
    let mut u16_3: u16 = crate::offset_date_time::OffsetDateTime::ordinal(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3328() {
    rusty_monitor::set_test_id(3328);
    let mut u16_0: u16 = 16u16;
    let mut i32_0: i32 = -151i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_0: i64 = 23i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut u32_0: u32 = 47u32;
    let mut u8_0: u8 = 36u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 6u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_1);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut u16_1: u16 = 35u16;
    let mut i32_1: i32 = -10i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = 126i8;
    let mut i8_1: i8 = -26i8;
    let mut i8_2: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 67i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u32_1: u32 = 32u32;
    let mut u8_3: u8 = 31u8;
    let mut u8_4: u8 = 55u8;
    let mut u8_5: u8 = 59u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = 79i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut u16_2: u16 = 75u16;
    let mut i32_2: i32 = -21i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_2);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_5, time_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_0);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_2);
    let mut u32_2: u32 = 44u32;
    let mut u8_6: u8 = 46u8;
    let mut u8_7: u8 = 31u8;
    let mut u8_8: u8 = 70u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_9: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_9);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_4);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut i32_3: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_5);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut month_3: month::Month = std::clone::Clone::clone(month_1_ref_0);
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut tuple_1: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_nano(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4049() {
    rusty_monitor::set_test_id(4049);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 101i64;
    let mut i64_1: i64 = 101i64;
    let mut i64_2: i64 = 13i64;
    let mut str_0: &str = "LjpTvDiy5eJZl9";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut i64_3: i64 = -30i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_0: u16 = 35u16;
    let mut i32_0: i32 = -10i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i8_0: i8 = 126i8;
    let mut i8_1: i8 = -26i8;
    let mut i8_2: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_4: i64 = 67i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut u32_0: u32 = 32u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 59u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_5: i64 = 79i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_3: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_3);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_sub(instant_0, duration_0);
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7938() {
    rusty_monitor::set_test_id(7938);
    let mut i64_0: i64 = 13i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut u32_0: u32 = 95u32;
    let mut u8_0: u8 = 47u8;
    let mut u8_1: u8 = 41u8;
    let mut u8_2: u8 = 12u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 2u16;
    let mut i32_0: i32 = -50i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut u16_1: u16 = 35u16;
    let mut i32_1: i32 = -10i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = 126i8;
    let mut i8_1: i8 = -26i8;
    let mut i8_2: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 67i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u32_1: u32 = 32u32;
    let mut u8_3: u8 = 31u8;
    let mut u8_4: u8 = 55u8;
    let mut u8_5: u8 = 59u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_2: i64 = 79i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut u16_2: u16 = 75u16;
    let mut i32_2: i32 = -21i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_1);
    let mut u32_2: u32 = 44u32;
    let mut u8_6: u8 = 46u8;
    let mut u8_7: u8 = 31u8;
    let mut u8_8: u8 = 70u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut month_1: month::Month = crate::month::Month::October;
    let mut u8_9: u8 = 86u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_9);
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_3);
    let mut month_3: month::Month = crate::month::Month::April;
    let mut i32_3: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_3);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(month_0_ref_0);
    let mut month_4: month::Month = crate::month::Month::previous(month_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_691() {
    rusty_monitor::set_test_id(691);
    let mut u16_0: u16 = 35u16;
    let mut i32_0: i32 = -15i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_1: i64 = 79i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_1: u16 = 75u16;
    let mut i32_1: i32 = -21i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut u32_0: u32 = 44u32;
    let mut u8_0: u8 = 46u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 70u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_3: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_3);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_1);
    let mut month_2: month::Month = crate::month::Month::April;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6034() {
    rusty_monitor::set_test_id(6034);
    let mut i32_0: i32 = -18i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_0);
    let mut i8_0: i8 = 65i8;
    let mut i8_1: i8 = -30i8;
    let mut i8_2: i8 = -12i8;
    let mut u16_0: u16 = 35u16;
    let mut i32_1: i32 = -10i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_3: i8 = 126i8;
    let mut i8_4: i8 = -26i8;
    let mut i8_5: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 32u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 59u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 79i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_1: u16 = 75u16;
    let mut i32_2: i32 = -21i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_1, time_0);
    let mut u32_1: u32 = 44u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_6: u8 = 6u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_2);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut i32_3: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_3);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5417() {
    rusty_monitor::set_test_id(5417);
    let mut month_0: month::Month = crate::month::Month::February;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i32_0: i32 = -81i32;
    let mut u16_0: u16 = 35u16;
    let mut i32_1: i32 = -10i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i8_0: i8 = 126i8;
    let mut i8_1: i8 = -26i8;
    let mut i8_2: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u16_1: u16 = 75u16;
    let mut i32_2: i32 = -21i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut u32_0: u32 = 44u32;
    let mut u8_0: u8 = 46u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 70u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut month_2: month::Month = crate::month::Month::October;
    let mut u8_3: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_3);
    let mut month_3: month::Month = crate::month::Month::next(month_2);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_1);
    let mut month_4: month::Month = crate::month::Month::April;
    let mut u8_4: u8 = crate::util::days_in_year_month(i32_0, month_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6763() {
    rusty_monitor::set_test_id(6763);
    let mut i64_0: i64 = 143i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i8_0: i8 = 65i8;
    let mut i8_1: i8 = -30i8;
    let mut i8_2: i8 = -12i8;
    let mut u16_0: u16 = 35u16;
    let mut i32_0: i32 = -10i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i8_3: i8 = 126i8;
    let mut i8_4: i8 = -26i8;
    let mut i8_5: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = 67i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u32_0: u32 = 32u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 59u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = 79i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_3: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_3);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5906() {
    rusty_monitor::set_test_id(5906);
    let mut u16_0: u16 = 35u16;
    let mut i32_0: i32 = -10i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i8_0: i8 = 126i8;
    let mut i8_1: i8 = -26i8;
    let mut i8_2: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 32u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 59u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 79i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_1: u16 = 75u16;
    let mut i32_1: i32 = -21i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut u32_1: u32 = 44u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_6: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_2);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut i32_2: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_1);
    let mut month_3: month::Month = crate::month::Month::previous(month_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5036() {
    rusty_monitor::set_test_id(5036);
    let mut i8_0: i8 = -23i8;
    let mut i8_1: i8 = -30i8;
    let mut i8_2: i8 = -12i8;
    let mut u16_0: u16 = 35u16;
    let mut i32_0: i32 = -10i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i8_3: i8 = 126i8;
    let mut i8_4: i8 = -26i8;
    let mut i8_5: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 32u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 0u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 79i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_1: u16 = 75u16;
    let mut i32_1: i32 = -21i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut u32_1: u32 = 44u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 3u8;
    let mut u8_5: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_6: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_2);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut i32_2: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_1);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7416() {
    rusty_monitor::set_test_id(7416);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_0: i64 = 99i64;
    let mut i8_0: i8 = 65i8;
    let mut i8_1: i8 = -30i8;
    let mut i8_2: i8 = -12i8;
    let mut u16_0: u16 = 35u16;
    let mut i32_0: i32 = -10i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i8_3: i8 = 126i8;
    let mut i8_4: i8 = -26i8;
    let mut i8_5: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 32u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 59u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 79i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_1: u16 = 75u16;
    let mut i32_1: i32 = -21i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_1, time_0);
    let mut u32_1: u32 = 44u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_6: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_2);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut i32_2: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_2);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u16_2: u16 = crate::offset_date_time::OffsetDateTime::millisecond(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1871() {
    rusty_monitor::set_test_id(1871);
    let mut u16_0: u16 = 80u16;
    let mut i32_0: i32 = 44i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i8_0: i8 = 65i8;
    let mut i8_1: i8 = -30i8;
    let mut i8_2: i8 = -12i8;
    let mut u16_1: u16 = 35u16;
    let mut i32_1: i32 = -10i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i8_3: i8 = 126i8;
    let mut i8_4: i8 = -26i8;
    let mut i8_5: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 32u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 59u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 79i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u32_1: u32 = 44u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_6: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_2);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u8_7: u8 = crate::date::Date::sunday_based_week(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4172() {
    rusty_monitor::set_test_id(4172);
    let mut u16_0: u16 = 35u16;
    let mut i32_0: i32 = -10i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i8_0: i8 = 126i8;
    let mut i8_1: i8 = -26i8;
    let mut i8_2: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 32u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 59u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 79i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_1: u16 = 75u16;
    let mut i32_1: i32 = -21i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut u32_1: u32 = 44u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut month_0: month::Month = crate::month::Month::June;
    let mut u8_6: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_2);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut i32_2: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1420() {
    rusty_monitor::set_test_id(1420);
    let mut u16_0: u16 = 35u16;
    let mut i32_0: i32 = -10i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 32u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 59u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 79i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_1: u16 = 75u16;
    let mut i32_1: i32 = -21i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_0);
    let mut u32_1: u32 = 44u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_6: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_2);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut tuple_1: (i32, u16) = crate::primitive_date_time::PrimitiveDateTime::to_ordinal_date(primitivedatetime_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4114() {
    rusty_monitor::set_test_id(4114);
    let mut i32_0: i32 = 269i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_1: i32 = -129i32;
    let mut i64_0: i64 = -130i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut u16_0: u16 = 63u16;
    let mut i32_2: i32 = 123i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut month_0: month::Month = crate::month::Month::June;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut i8_0: i8 = 22i8;
    let mut i8_1: i8 = 65i8;
    let mut i8_2: i8 = -30i8;
    let mut i8_3: i8 = -12i8;
    let mut i8_4: i8 = -26i8;
    let mut i8_5: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_0, i8_4);
    let mut i64_1: i64 = 67i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut u32_0: u32 = 32u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 59u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_2: i64 = 79i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut u16_1: u16 = 75u16;
    let mut i32_3: i32 = -21i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_4, time_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_0);
    let mut u32_1: u32 = 44u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut month_2: month::Month = crate::month::Month::October;
    let mut u8_6: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_3: month::Month = crate::month::Month::next(month_2);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_2);
    let mut month_4: month::Month = crate::month::Month::April;
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_3, i8_2, i8_1);
    let mut month_3_ref_0: &month::Month = &mut month_3;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_3_ref_0, month_1_ref_0);
    let mut tuple_1: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_1);
    let mut tuple_2: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_nano(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3171() {
    rusty_monitor::set_test_id(3171);
    let mut i8_0: i8 = 65i8;
    let mut i8_1: i8 = -30i8;
    let mut i8_2: i8 = -12i8;
    let mut u16_0: u16 = 35u16;
    let mut i32_0: i32 = -10i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_0);
    let mut i8_3: i8 = 126i8;
    let mut i8_4: i8 = -26i8;
    let mut i8_5: i8 = 91i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = 67i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u32_0: u32 = 32u32;
    let mut u8_0: u8 = 31u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 59u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 79i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u16_1: u16 = 75u16;
    let mut i32_1: i32 = -21i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut u32_1: u32 = 44u32;
    let mut u8_3: u8 = 46u8;
    let mut u8_4: u8 = 31u8;
    let mut u8_5: u8 = 70u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut u8_6: u8 = 18u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_2);
    let mut month_2: month::Month = crate::month::Month::April;
    let mut i32_2: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_1);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u8_7: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_1);
    panic!("From RustyUnit with love");
}
}