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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1343() {
    rusty_monitor::set_test_id(1343);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_0);
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut i32_0: i32 = 13i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_1: i32 = -31i32;
    let mut i64_0: i64 = -60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut i32_2: i32 = 139i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_0};
    let mut u16_0: u16 = 49u16;
    let mut i32_3: i32 = 0i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut i64_1: i64 = -124i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut u32_0: u32 = 25u32;
    let mut u8_0: u8 = 53u8;
    let mut u8_1: u8 = 86u8;
    let mut u8_2: u8 = 60u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_1: u16 = 55u16;
    let mut i32_4: i32 = -99i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_4, time_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_1);
    let mut i64_2: i64 = 237i64;
    let mut i64_3: i64 = 120i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i32_5: i32 = 94i32;
    let mut i64_4: i64 = 43i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_5);
    let mut i32_6: i32 = 108i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_6);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_5);
    let mut f64_0: f64 = 173.164470f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut month_1: month::Month = crate::month::Month::November;
    let mut f32_0: f32 = crate::duration::Duration::as_seconds_f32(duration_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut month_2: month::Month = crate::month::Month::May;
    let mut u32_1: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_2);
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_3);
    let mut tuple_1: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_0);
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut tuple_2: () = std::cmp::Eq::assert_receiver_is_total_eq(month_2_ref_0);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_1_ref_0, month_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3515() {
    rusty_monitor::set_test_id(3515);
    let mut u16_0: u16 = 42u16;
    let mut u8_0: u8 = 57u8;
    let mut u8_1: u8 = 47u8;
    let mut u8_2: u8 = 41u8;
    let mut i8_0: i8 = -18i8;
    let mut i8_1: i8 = 50i8;
    let mut i8_2: i8 = 16i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = -83i8;
    let mut i8_4: i8 = 113i8;
    let mut i8_5: i8 = -59i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 56u32;
    let mut u8_3: u8 = 26u8;
    let mut u8_4: u8 = 11u8;
    let mut u8_5: u8 = 39u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i64_0: i64 = 56i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i8_6: i8 = -56i8;
    let mut i8_7: i8 = 16i8;
    let mut i8_8: i8 = -21i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_1: i64 = 178i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i8_9: i8 = 52i8;
    let mut i8_10: i8 = -30i8;
    let mut i8_11: i8 = -124i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 19i8;
    let mut i8_13: i8 = -108i8;
    let mut i8_14: i8 = 79i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_2: i64 = 93i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut u32_1: u32 = 49u32;
    let mut u8_6: u8 = 63u8;
    let mut u8_7: u8 = 35u8;
    let mut u8_8: u8 = 30u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_1);
    let mut i8_15: i8 = -5i8;
    let mut i8_16: i8 = -63i8;
    let mut i8_17: i8 = -13i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = -89i8;
    let mut i8_19: i8 = 22i8;
    let mut i8_20: i8 = 51i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i64_3: i64 = 69i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut u8_9: u8 = 22u8;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut i32_0: i32 = -21i32;
    let mut u8_10: u8 = 84u8;
    let mut u8_11: u8 = 13u8;
    let mut u8_12: u8 = 74u8;
    let mut i64_4: i64 = 126i64;
    let mut i32_1: i32 = 13i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_3, date_1);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut i32_2: i32 = -31i32;
    let mut i64_5: i64 = -60i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_2);
    let mut i32_3: i32 = 139i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_2};
    let mut u16_1: u16 = 49u16;
    let mut i32_4: i32 = 0i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut u32_2: u32 = 25u32;
    let mut u8_13: u8 = 53u8;
    let mut u8_14: u8 = 86u8;
    let mut u8_15: u8 = 60u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_15, u8_14, u8_13, u32_2);
    let mut u16_2: u16 = 55u16;
    let mut i32_5: i32 = -99i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_5, time_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_5);
    let mut i64_6: i64 = 237i64;
    let mut i64_7: i64 = 120i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_7);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut i32_6: i32 = 94i32;
    let mut i64_8: i64 = 43i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_8, i32_6);
    let mut i32_7: i32 = 108i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_8, i32_7);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_6, duration_9);
    let mut date_6: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_7);
    let mut date_7: crate::date::Date = crate::date::Date::saturating_sub(date_6, duration_7);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_7);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_3, utcoffset_8);
    let mut time_4: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_4);
    let mut f64_0: f64 = 176.025602f64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_10_ref_0: &mut crate::duration::Duration = &mut duration_10;
    let mut month_1: month::Month = crate::month::Month::November;
    let mut tuple_0: (u8, u8, u8) = crate::time::Time::as_hms(time_4);
    let mut f32_0: f32 = crate::duration::Duration::as_seconds_f32(duration_6);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::hours(i64_6);
    let mut month_2: month::Month = crate::month::Month::May;
    let mut u32_3: u32 = crate::primitive_date_time::PrimitiveDateTime::microsecond(primitivedatetime_2);
    let mut tuple_1: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_4);
    let mut tuple_2: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_12, u8_11, u8_10);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_0, u8_9);
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut month_3: month::Month = std::clone::Clone::clone(month_2_ref_0);
    let mut i8_21: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_7);
    let mut result_2: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_2, u8_1, u8_0, u16_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6008() {
    rusty_monitor::set_test_id(6008);
    let mut i64_0: i64 = 61i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut u32_0: u32 = 1u32;
    let mut u8_0: u8 = 51u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 3u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u8_3: u8 = 68u8;
    let mut u32_1: u32 = 31u32;
    let mut u8_4: u8 = 35u8;
    let mut u8_5: u8 = 1u8;
    let mut u8_6: u8 = 2u8;
    let mut i64_1: i64 = 28i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_2: u32 = 78u32;
    let mut u8_7: u8 = 45u8;
    let mut u8_8: u8 = 47u8;
    let mut u8_9: u8 = 1u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_9, u8_8, u8_7, u32_2);
    let mut i32_0: i32 = -64i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_0);
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut i32_1: i32 = 13i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_1, date_1);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_2: i32 = -31i32;
    let mut i64_2: i64 = -60i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut i32_3: i32 = 139i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_2};
    let mut u16_0: u16 = 49u16;
    let mut i32_4: i32 = 0i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut i64_3: i64 = -124i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut u16_1: u16 = 55u16;
    let mut i32_5: i32 = -99i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut i64_4: i64 = 233i64;
    let mut i64_5: i64 = 120i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut i32_6: i32 = 108i32;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_6: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_6, i32_6);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_7);
    let mut date_6: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut f64_0: f64 = 173.164470f64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    let mut month_1: month::Month = crate::month::Month::November;
    let mut f32_0: f32 = crate::duration::Duration::as_seconds_f32(duration_5);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut month_2: month::Month = crate::month::Month::May;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_4);
    let mut tuple_1: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_0);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut tuple_2: () = std::cmp::Eq::assert_receiver_is_total_eq(month_2_ref_0);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_1_ref_0, month_0_ref_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_2);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_6, u8_5, u8_4, u32_1);
    let mut result_1: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_3);
    let mut tuple_3: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6586() {
    rusty_monitor::set_test_id(6586);
    let mut u32_0: u32 = 31u32;
    let mut u8_0: u8 = 35u8;
    let mut u8_1: u8 = 3u8;
    let mut u8_2: u8 = 2u8;
    let mut i64_0: i64 = 28i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_1: u32 = 78u32;
    let mut u8_3: u8 = 45u8;
    let mut u8_4: u8 = 47u8;
    let mut u8_5: u8 = 1u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_0: i32 = -64i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut i32_1: i32 = 13i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_2: i32 = -31i32;
    let mut i64_1: i64 = -60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut i32_3: i32 = 139i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_1};
    let mut u16_0: u16 = 49u16;
    let mut i32_4: i32 = 0i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut i64_2: i64 = -124i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut u16_1: u16 = 55u16;
    let mut i32_5: i32 = -99i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut i64_3: i64 = 233i64;
    let mut i64_4: i64 = 120i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i32_6: i32 = 94i32;
    let mut i64_5: i64 = 43i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_6);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f64_0: f64 = 173.164470f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut month_0: month::Month = crate::month::Month::November;
    let mut f32_0: f32 = crate::duration::Duration::as_seconds_f32(duration_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut month_1: month::Month = crate::month::Month::May;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_4);
    let mut tuple_1: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_0);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut tuple_2: () = std::cmp::Eq::assert_receiver_is_total_eq(month_1_ref_0);
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}
}