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
fn rusty_test_1513() {
    rusty_monitor::set_test_id(1513);
    let mut i32_0: i32 = -17i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u32_0: u32 = 90u32;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 86u8;
    let mut u8_2: u8 = 22u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = 181i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i32_1: i32 = 96i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut month_2: month::Month = crate::month::Month::December;
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut month_3: month::Month = crate::month::Month::December;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_2_ref_0, month_1_ref_0);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4812() {
    rusty_monitor::set_test_id(4812);
    let mut u8_0: u8 = 96u8;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 24u8;
    let mut i32_0: i32 = 239i32;
    let mut i32_1: i32 = -163i32;
    let mut i64_0: i64 = -151i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i32_2: i32 = -50i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_1);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut month_0: month::Month = crate::month::Month::June;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = std::clone::Clone::clone(month_0_ref_0);
    let mut u8_3: u8 = crate::time::Time::minute(time_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_2, u8_1, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4037() {
    rusty_monitor::set_test_id(4037);
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -169i64;
    let mut i64_1: i64 = 96i64;
    let mut i64_2: i64 = 138i64;
    let mut str_0: &str = "";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut u32_0: u32 = 21u32;
    let mut u8_0: u8 = 25u8;
    let mut u8_1: u8 = 3u8;
    let mut u8_2: u8 = 72u8;
    let mut i64_3: i64 = 116i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_3);
    let mut i32_0: i32 = 34i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i64_4: i64 = -190i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_5: i64 = 67i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_2: month::Month = crate::month::Month::previous(month_1);
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(month_2_ref_0);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_410() {
    rusty_monitor::set_test_id(410);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut i8_0: i8 = -31i8;
    let mut i8_1: i8 = 9i8;
    let mut i8_2: i8 = 34i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_0: i32 = 76i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut i32_1: i32 = -144i32;
    let mut month_0: month::Month = crate::month::Month::April;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::January;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_1_ref_0, month_0_ref_0);
    let mut u16_0: u16 = crate::util::days_in_year(i32_1);
    let mut month_2: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_694() {
    rusty_monitor::set_test_id(694);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_0: i32 = -86i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut i8_0: i8 = 58i8;
    let mut i8_1: i8 = -30i8;
    let mut i8_2: i8 = -62i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_1, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i32_1: i32 = 118i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_1};
    let mut i64_0: i64 = -79i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4666() {
    rusty_monitor::set_test_id(4666);
    let mut i64_0: i64 = 41i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i32_0: i32 = 159i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut i8_0: i8 = -90i8;
    let mut i8_1: i8 = -91i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = 104i8;
    let mut i8_4: i8 = 51i8;
    let mut i8_5: i8 = 106i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 96u32;
    let mut u8_0: u8 = 64u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 86u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 75i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut month_0: month::Month = crate::month::Month::May;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut month_1: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_2);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_1_ref_0, month_0_ref_0);
    let mut u8_3: u8 = crate::weekday::Weekday::number_from_monday(weekday_0);
    let mut tuple_0: (u8, u8, u8, u16) = crate::offset_date_time::OffsetDateTime::to_hms_milli(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4511() {
    rusty_monitor::set_test_id(4511);
    let mut i128_0: i128 = 108i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut u32_0: u32 = 23u32;
    let mut u8_0: u8 = 74u8;
    let mut u8_1: u8 = 92u8;
    let mut u8_2: u8 = 12u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f64_0: f64 = -128.257766f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i8_0: i8 = 53i8;
    let mut i8_1: i8 = 114i8;
    let mut i8_2: i8 = 7i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = -66i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i8_3: i8 = 72i8;
    let mut i8_4: i8 = -47i8;
    let mut i8_5: i8 = -13i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i128_1: i128 = 121i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i8_6: i8 = -42i8;
    let mut i8_7: i8 = -40i8;
    let mut i8_8: i8 = 22i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u16_0: u16 = 70u16;
    let mut i32_0: i32 = 171i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_2);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i32_1: i32 = 177i32;
    let mut i64_1: i64 = -130i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut u32_1: u32 = 76u32;
    let mut u8_3: u8 = 66u8;
    let mut u8_4: u8 = 83u8;
    let mut u8_5: u8 = 95u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u16_1: u16 = 64u16;
    let mut i32_2: i32 = -48i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_2};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_2, utcoffset_3);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u8_6: u8 = 52u8;
    let mut i32_3: i32 = 83i32;
    let mut u8_7: u8 = 10u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_7);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_3, u8_6, weekday_0);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_3, duration_5);
    let mut tuple_0: (util::DateAdjustment, crate::time::Time) = crate::time::Time::adjusting_sub(time_1, duration_3);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut u8_8: u8 = crate::primitive_date_time::PrimitiveDateTime::minute(primitivedatetime_4);
    let mut month_2: month::Month = crate::date::Date::month(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2078() {
    rusty_monitor::set_test_id(2078);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_0: i32 = 37i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut i64_0: i64 = 23i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut i64_1: i64 = 4i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut u16_0: u16 = 44u16;
    let mut i32_1: i32 = -242i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_3);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut i64_2: i64 = -68i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_2);
    let mut u8_0: u8 = 8u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_0);
    let mut i16_0: i16 = crate::duration::Duration::subsec_milliseconds(duration_4);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_0, duration_2);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::millisecond(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2125() {
    rusty_monitor::set_test_id(2125);
    let mut i64_0: i64 = -212i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = -153i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i32_0: i32 = -72i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i32_1: i32 = 63i32;
    let mut u8_0: u8 = 47u8;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_0: u16 = 40u16;
    let mut i32_2: i32 = -102i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_nano(primitivedatetime_1);
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_0);
    let mut bool_0: bool = crate::util::is_leap_year(i32_1);
    let mut u8_1: u8 = crate::primitive_date_time::PrimitiveDateTime::minute(primitivedatetime_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_1, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2629() {
    rusty_monitor::set_test_id(2629);
    let mut u32_0: u32 = 57u32;
    let mut u8_0: u8 = 97u8;
    let mut u8_1: u8 = 61u8;
    let mut u8_2: u8 = 64u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut i32_0: i32 = -128i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u16_0: u16 = 46u16;
    let mut i32_1: i32 = -133i32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut month_1: month::Month = crate::date::Date::month(date_0);
    let mut u32_1: u32 = crate::offset_date_time::OffsetDateTime::microsecond(offsetdatetime_1);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::next_day(date_1);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut month_2: month::Month = std::clone::Clone::clone(month_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2742() {
    rusty_monitor::set_test_id(2742);
    let mut i8_0: i8 = 70i8;
    let mut bool_0: bool = false;
    let mut i64_0: i64 = -81i64;
    let mut i64_1: i64 = 115i64;
    let mut i64_2: i64 = 32i64;
    let mut str_0: &str = "P";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut i32_0: i32 = 187i32;
    let mut u32_0: u32 = 81u32;
    let mut str_1: &str = crate::error::component_range::ComponentRange::name(componentrange_0);
    let mut month_0: month::Month = crate::month::Month::March;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3549() {
    rusty_monitor::set_test_id(3549);
    let mut i64_0: i64 = 29i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i64_1: i64 = 189i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i64_2: i64 = -72i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::January;
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut month_2: month::Month = crate::month::Month::April;
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_2_ref_0, month_1_ref_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut i64_3: i64 = crate::duration::Duration::whole_seconds(duration_4);
    let mut f64_0: f64 = crate::duration::Duration::as_seconds_f64(duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_542() {
    rusty_monitor::set_test_id(542);
    let mut i128_0: i128 = 92i128;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut f32_0: f32 = 56.987908f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = -2i64;
    let mut i64_1: i64 = 65i64;
    let mut i64_2: i64 = -155i64;
    let mut str_0: &str = "lYUbpq8j";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut month_0: month::Month = crate::month::Month::December;
    let mut f32_1: f32 = 36.532755f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_0: i32 = 4i32;
    let mut f32_2: f32 = -47.407138f32;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_0);
    let mut duration_6: std::time::Duration = crate::duration::Duration::abs_std(duration_5);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut month_3: month::Month = std::clone::Clone::clone(month_2_ref_0);
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1090() {
    rusty_monitor::set_test_id(1090);
    let mut i64_0: i64 = 208i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = -187i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = 110i8;
    let mut i8_1: i8 = 39i8;
    let mut i8_2: i8 = 3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = -141i32;
    let mut i64_2: i64 = 47i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut i8_3: i8 = 127i8;
    let mut i8_4: i8 = 0i8;
    let mut i8_5: i8 = -9i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -17i8;
    let mut i8_7: i8 = 60i8;
    let mut i8_8: i8 = 27i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_3: i64 = -44i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut u32_0: u32 = 26u32;
    let mut u8_0: u8 = 13u8;
    let mut u8_1: u8 = 39u8;
    let mut u8_2: u8 = 98u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = -128i8;
    let mut i8_10: i8 = -41i8;
    let mut i8_11: i8 = -21i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 90i8;
    let mut i8_13: i8 = -106i8;
    let mut i8_14: i8 = -8i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i8_15: i8 = 26i8;
    let mut i8_16: i8 = 67i8;
    let mut i8_17: i8 = -81i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i64_4: i64 = 53i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut u32_1: u32 = 13u32;
    let mut u8_3: u8 = 25u8;
    let mut u8_4: u8 = 75u8;
    let mut u8_5: u8 = 83u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_5: i64 = -54i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut i8_18: i8 = 15i8;
    let mut i8_19: i8 = 32i8;
    let mut i8_20: i8 = -28i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i8_21: i8 = -48i8;
    let mut i8_22: i8 = 80i8;
    let mut i8_23: i8 = 57i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut u32_2: u32 = 7u32;
    let mut u8_6: u8 = 95u8;
    let mut u8_7: u8 = 82u8;
    let mut u8_8: u8 = 72u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_6: i64 = -173i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut u16_0: u16 = 93u16;
    let mut i32_1: i32 = -11i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_6);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_2};
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_7};
    let mut u8_9: u8 = 7u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_9);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut i128_0: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_2);
    let mut i8_24: i8 = crate::utc_offset::UtcOffset::seconds_past_minute(utcoffset_6);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::checked_sub(date_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_241() {
    rusty_monitor::set_test_id(241);
    let mut i64_0: i64 = -56i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut u32_0: u32 = 65u32;
    let mut u8_0: u8 = 68u8;
    let mut u8_1: u8 = 11u8;
    let mut u8_2: u8 = 17u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 181i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut i64_2: i64 = 6i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i64_3: i64 = -135i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut month_0: month::Month = crate::month::Month::March;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut month_3: month::Month = std::clone::Clone::clone(month_2_ref_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_5);
    let mut month_3_ref_0: &month::Month = &mut month_3;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(month_3_ref_0);
    let mut tuple_1: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_micro(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_560() {
    rusty_monitor::set_test_id(560);
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 38i8;
    let mut i8_2: i8 = 70i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = -153.239442f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_0: i64 = -21i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i32_0: i32 = 124i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_2);
    let mut u8_0: u8 = 51u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2201() {
    rusty_monitor::set_test_id(2201);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_0: i64 = 104i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i8_0: i8 = -112i8;
    let mut i8_1: i8 = 40i8;
    let mut i8_2: i8 = 110i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 26i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut u32_0: u32 = 73u32;
    let mut u8_0: u8 = 83u8;
    let mut u8_1: u8 = 17u8;
    let mut u8_2: u8 = 18u8;
    let mut month_0: month::Month = crate::month::Month::June;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = std::clone::Clone::clone(month_0_ref_0);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut tuple_0: (u8, u8, u8, u32) = crate::time::Time::as_hms_micro(time_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_sub(duration_1, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3356() {
    rusty_monitor::set_test_id(3356);
    let mut i32_0: i32 = -9i32;
    let mut i64_0: i64 = -11i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i8_0: i8 = -47i8;
    let mut i8_1: i8 = 29i8;
    let mut i8_2: i8 = -68i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 3u16;
    let mut i32_1: i32 = -2i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut i64_1: i64 = 61i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i64_2: i64 = -179i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut month_0: month::Month = crate::primitive_date_time::PrimitiveDateTime::month(primitivedatetime_1);
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(month_0_ref_0);
    panic!("From RustyUnit with love");
}
}