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
	use std::convert::TryFrom;
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3073() {
    rusty_monitor::set_test_id(3073);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i64_0: i64 = -204i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_0);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i8_0: i8 = 120i8;
    let mut i8_1: i8 = -11i8;
    let mut i8_2: i8 = -37i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_3, utcoffset_2);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut i32_0: i32 = 64i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_1);
    let mut i64_1: i64 = 30i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i64_2: i64 = 172i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_1: i32 = -14i32;
    let mut i64_3: i64 = 1i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut i64_4: i64 = -95i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_1, utcoffset_0);
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(month_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1796() {
    rusty_monitor::set_test_id(1796);
    let mut f64_0: f64 = 18.475403f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_0: i8 = 70i8;
    let mut i8_1: i8 = -52i8;
    let mut i8_2: i8 = -10i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_1: f64 = 13.242451f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i32_0: i32 = -91i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut f32_0: f32 = -123.842517f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_1: i32 = 22i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_2: i32 = 70i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_2, duration_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_3: i32 = -174i32;
    let mut f32_1: f32 = -58.694062f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_3);
    let mut i32_4: i32 = -63i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_4);
    let mut u32_0: u32 = 58u32;
    let mut u8_0: u8 = 67u8;
    let mut u8_1: u8 = 60u8;
    let mut u8_2: u8 = 82u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = -105i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_3: i8 = 10i8;
    let mut i8_4: i8 = -104i8;
    let mut i8_5: i8 = -58i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = -56i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut f64_2: f64 = -132.897149f64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut i8_6: i8 = 22i8;
    let mut i8_7: i8 = -49i8;
    let mut i8_8: i8 = -43i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u16_0: u16 = 78u16;
    let mut i32_5: i32 = 62i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut i32_6: i32 = -32i32;
    let mut i64_1: i64 = -7i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_6);
    let mut u16_1: u16 = 77u16;
    let mut i32_7: i32 = -44i32;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_1);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_5, duration_9);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_6);
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut u32_1: u32 = 88u32;
    let mut u8_3: u8 = 57u8;
    let mut u8_4: u8 = 94u8;
    let mut u8_5: u8 = 11u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_8: i32 = 48i32;
    let mut u8_6: u8 = 29u8;
    let mut month_0: month::Month = crate::month::Month::January;
    let mut i32_9: i32 = -47i32;
    let mut u8_7: u8 = 78u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_7);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_9, month_0, u8_6);
    let mut date_7: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_8);
    let mut duration_10: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u8_8: u8 = crate::primitive_date_time::PrimitiveDateTime::minute(primitivedatetime_3);
    let mut u32_2: u32 = crate::offset_date_time::OffsetDateTime::microsecond(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_46() {
    rusty_monitor::set_test_id(46);
    let mut i32_0: i32 = -32i32;
    let mut i64_0: i64 = -7i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut u16_0: u16 = 77u16;
    let mut i32_1: i32 = -44i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_0: u32 = 88u32;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 94u8;
    let mut u8_2: u8 = 11u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 33u16;
    let mut i32_2: i32 = 80i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut i32_3: i32 = 48i32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut u8_3: u8 = 29u8;
    let mut month_0: month::Month = crate::month::Month::January;
    let mut i32_4: i32 = -47i32;
    let mut u8_4: u8 = 78u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_4);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_4, month_0, u8_3);
    let mut u8_5: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_1, utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6955() {
    rusty_monitor::set_test_id(6955);
    let mut i8_0: i8 = 19i8;
    let mut i8_1: i8 = 55i8;
    let mut i8_2: i8 = -85i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u16_0: u16 = 93u16;
    let mut i32_0: i32 = 5i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut i8_3: i8 = -5i8;
    let mut i8_4: i8 = 14i8;
    let mut i8_5: i8 = 43i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_1: i32 = 153i32;
    let mut i64_0: i64 = -122i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut u32_0: u32 = 11u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 16u8;
    let mut u8_2: u8 = 65u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 55i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i64_2: i64 = -24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut i32_2: i32 = -50i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_1);
    let mut i32_3: i32 = -68i32;
    let mut f32_0: f32 = -123.842517f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_4: i32 = 22i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_5: i32 = 70i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_4, time: time_1};
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_4, duration_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_6: i32 = -174i32;
    let mut f32_1: f32 = -58.694062f32;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_6);
    let mut i32_7: i32 = -63i32;
    let mut date_5: crate::date::Date = crate::date::Date {value: i32_7};
    let mut date_6: crate::date::Date = crate::date::Date::saturating_add(date_5, duration_6);
    let mut u32_1: u32 = 58u32;
    let mut u8_3: u8 = 67u8;
    let mut u8_4: u8 = 60u8;
    let mut u8_5: u8 = 82u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i128_0: i128 = -105i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_6: i8 = 10i8;
    let mut i8_7: i8 = -104i8;
    let mut i8_8: i8 = -58i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_3: i64 = -56i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut f64_0: f64 = -132.897149f64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_9, duration_8);
    let mut i8_9: i8 = 22i8;
    let mut i8_10: i8 = -49i8;
    let mut i8_11: i8 = -43i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = -28i8;
    let mut i8_13: i8 = -2i8;
    let mut i8_14: i8 = -6i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u16_1: u16 = 78u16;
    let mut i32_8: i32 = 62i32;
    let mut date_7: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_8, u16_1);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_7);
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_6, utcoffset_4);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_7);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_3, date_6);
    let mut i32_9: i32 = -32i32;
    let mut i64_4: i64 = -7i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_9);
    let mut u16_2: u16 = 77u16;
    let mut i32_10: i32 = -44i32;
    let mut date_8: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_10, u16_2);
    let mut primitivedatetime_8: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_8);
    let mut primitivedatetime_9: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_8, duration_11);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_9);
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut u16_3: u16 = 33u16;
    let mut i32_11: i32 = 80i32;
    let mut date_9: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_11, u16_3);
    let mut i32_12: i32 = 48i32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut u8_6: u8 = 29u8;
    let mut month_0: month::Month = crate::month::Month::January;
    let mut i32_13: i32 = -47i32;
    let mut u8_7: u8 = 78u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_7);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_13, month_0, u8_6);
    let mut u8_8: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    let mut date_10: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_12);
    let mut tuple_0: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_4);
    let mut duration_12: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u8_9: u8 = crate::primitive_date_time::PrimitiveDateTime::minute(primitivedatetime_5);
    let mut result_2: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_3);
    let mut i32_14: i32 = crate::offset_date_time::OffsetDateTime::year(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_713() {
    rusty_monitor::set_test_id(713);
    let mut f64_0: f64 = -3.593293f64;
    let mut i128_0: i128 = -83i128;
    let mut i64_0: i64 = -141i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u16_0: u16 = 41u16;
    let mut i32_0: i32 = -82i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i32_1: i32 = 198i32;
    let mut f32_0: f32 = -15.548496f32;
    let mut i64_1: i64 = -5i64;
    let mut i64_2: i64 = 113i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_3: i64 = -22i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i64_4: i64 = 17i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::weeks(i64_4);
    let mut i32_2: i32 = 154i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_3);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i64_5: i64 = 31i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i32_3: i32 = -99i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_5, time_0);
    let mut i8_0: i8 = 10i8;
    let mut i32_4: i32 = -5i32;
    let mut i64_6: i64 = 41i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_3, i32_4);
    let mut i8_1: i8 = -27i8;
    let mut u32_0: u32 = 34u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 3u8;
    let mut u8_2: u8 = 49u8;
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f32_1: f32 = -123.842517f32;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_5: i32 = 22i32;
    let mut date_6: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_3, date_6);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_6: i32 = -174i32;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_5, i32_5);
    let mut i32_7: i32 = -63i32;
    let mut date_7: crate::date::Date = crate::date::Date {value: i32_6};
    let mut date_8: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_6);
    let mut u8_3: u8 = 67u8;
    let mut u8_4: u8 = 60u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_1, u8_4, u32_0);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_7: i64 = -56i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_8);
    let mut i8_2: i8 = -28i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_0, i8_1);
    let mut u16_1: u16 = 78u16;
    let mut i32_8: i32 = 62i32;
    let mut date_9: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_8);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_5, date_9);
    let mut i32_9: i32 = -32i32;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_9);
    let mut u16_2: u16 = 77u16;
    let mut i32_10: i32 = -44i32;
    let mut date_10: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_8, u16_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_7);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_2);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_6);
    let mut u32_1: u32 = 88u32;
    let mut u8_5: u8 = 94u8;
    let mut u8_6: u8 = 11u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_2, u8_0, u32_1);
    let mut u16_3: u16 = 33u16;
    let mut date_11: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_10, u16_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_10, time_3);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut month_0: month::Month = crate::month::Month::January;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_6);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4227() {
    rusty_monitor::set_test_id(4227);
    let mut i32_0: i32 = -68i32;
    let mut f32_0: f32 = -123.842517f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_1: i32 = 22i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_2: i32 = 70i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_3: i32 = -174i32;
    let mut f32_1: f32 = -58.694062f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_3);
    let mut i32_4: i32 = -63i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut u32_0: u32 = 58u32;
    let mut u8_0: u8 = 67u8;
    let mut u8_1: u8 = 60u8;
    let mut u8_2: u8 = 82u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = -105i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_0: i8 = 10i8;
    let mut i8_1: i8 = -104i8;
    let mut i8_2: i8 = -58i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = -56i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut f64_0: f64 = -132.897149f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut i8_3: i8 = -28i8;
    let mut i8_4: i8 = -2i8;
    let mut i8_5: i8 = -6i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u16_0: u16 = 78u16;
    let mut i32_5: i32 = 62i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_3);
    let mut i32_6: i32 = -32i32;
    let mut i64_1: i64 = -7i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_6);
    let mut u16_1: u16 = 77u16;
    let mut i32_7: i32 = -44i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_1);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_4, duration_7);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_5);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_4);
    let mut u16_2: u16 = 33u16;
    let mut i32_8: i32 = 80i32;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_8, u16_2);
    let mut i32_9: i32 = 48i32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut u8_3: u8 = 29u8;
    let mut month_0: month::Month = crate::month::Month::January;
    let mut i32_10: i32 = -47i32;
    let mut u8_4: u8 = 78u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_4);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_10, month_0, u8_3);
    let mut u8_5: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    let mut date_7: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_9);
    let mut tuple_0: (i32, u16) = crate::offset_date_time::OffsetDateTime::to_ordinal_date(offsetdatetime_3);
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u8_6: u8 = crate::primitive_date_time::PrimitiveDateTime::minute(primitivedatetime_1);
    let mut result_2: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
    panic!("From RustyUnit with love");
}
}