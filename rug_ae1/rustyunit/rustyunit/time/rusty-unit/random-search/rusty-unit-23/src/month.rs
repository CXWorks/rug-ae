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
fn rusty_test_2661() {
    rusty_monitor::set_test_id(2661);
    let mut u32_0: u32 = 49u32;
    let mut u8_0: u8 = 17u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 64u8;
    let mut i32_0: i32 = -130i32;
    let mut f32_0: f32 = -15.040911f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut u8_3: u8 = 63u8;
    let mut i64_0: i64 = 28i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut month_0: month::Month = crate::month::Month::July;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = std::clone::Clone::clone(month_0_ref_0);
    let mut f32_1: f32 = crate::duration::Duration::as_seconds_f32(duration_1);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4488() {
    rusty_monitor::set_test_id(4488);
    let mut f64_0: f64 = 90.287653f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 120i32;
    let mut i64_0: i64 = -141i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut f32_0: f32 = -6.499217f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut f32_1: f32 = -131.613306f32;
    let mut month_0: month::Month = crate::month::Month::July;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::January;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_2_ref_0, month_0_ref_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut i64_1: i64 = crate::duration::Duration::whole_seconds(duration_3);
    let mut u8_0: u8 = crate::weekday::Weekday::number_from_monday(weekday_0);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4373() {
    rusty_monitor::set_test_id(4373);
    let mut i64_0: i64 = -59i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut u16_0: u16 = 23u16;
    let mut i32_0: i32 = 151i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut i32_1: i32 = 82i32;
    let mut i32_2: i32 = 20i32;
    let mut f64_0: f64 = 126.040733f64;
    let mut i64_1: i64 = 10i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut u32_0: u32 = 68u32;
    let mut f64_1: f64 = 127.899716f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(month_0_ref_0);
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::iso_week(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1647() {
    rusty_monitor::set_test_id(1647);
    let mut f64_0: f64 = -20.924859f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_0: i64 = 123i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i64_1: i64 = 62i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut i64_2: i64 = 194i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut u32_0: u32 = 7u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 14u8;
    let mut u8_2: u8 = 51u8;
    let mut u8_3: u8 = 35u8;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_3);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
    let mut time_0: crate::time::Time = std::result::Result::unwrap(result_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_3);
    let mut i128_0: i128 = crate::duration::Duration::whole_nanoseconds(duration_2);
    let mut month_0: month::Month = std::result::Result::unwrap(result_0);
    let mut i128_1: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4361() {
    rusty_monitor::set_test_id(4361);
    let mut i32_0: i32 = 109i32;
    let mut i64_0: i64 = -138i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i32_1: i32 = -7i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_2: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_0: u32 = 13u32;
    let mut u8_0: u8 = 89u8;
    let mut u8_1: u8 = 85u8;
    let mut u8_2: u8 = 4u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_1: i64 = -82i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::abs(duration_3);
    let mut i32_2: i32 = 33i32;
    let mut i64_2: i64 = -210i64;
    let mut u8_3: u8 = 9u8;
    let mut i32_3: i32 = 63i32;
    let mut month_0: month::Month = crate::month::Month::April;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_3);
    let mut result_1: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::checked_sub(date_0, duration_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1651() {
    rusty_monitor::set_test_id(1651);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u8_0: u8 = 64u8;
    let mut i32_0: i32 = 119i32;
    let mut u32_0: u32 = 60u32;
    let mut u8_1: u8 = 89u8;
    let mut u8_2: u8 = 6u8;
    let mut u8_3: u8 = 3u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i64_0: i64 = -114i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut i64_1: i64 = 4i64;
    let mut i32_1: i32 = -107i32;
    let mut i64_2: i64 = 7i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut u32_1: u32 = 78u32;
    let mut u8_4: u8 = 72u8;
    let mut u8_5: u8 = 42u8;
    let mut u8_6: u8 = 31u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_5, u8_4, u32_1);
    let mut u8_7: u8 = 69u8;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::previous(weekday_2);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut result_0: std::result::Result<month::Month, crate::error::component_range::ComponentRange> = std::convert::TryFrom::try_from(u8_7);
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut tuple_0: (util::DateAdjustment, crate::time::Time) = crate::time::Time::adjusting_add(time_1, duration_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut tuple_1: (u8, u8, u8, u32) = crate::primitive_date_time::PrimitiveDateTime::as_hms_nano(primitivedatetime_0);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_0, u8_0, weekday_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1445() {
    rusty_monitor::set_test_id(1445);
    let mut u32_0: u32 = 54u32;
    let mut u8_0: u8 = 25u8;
    let mut u8_1: u8 = 47u8;
    let mut u8_2: u8 = 95u8;
    let mut i32_0: i32 = -22i32;
    let mut i64_0: i64 = 151i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_0);
    let mut i128_0: i128 = 166i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_1: i64 = -103i64;
    let mut month_0: month::Month = crate::month::Month::April;
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp(i64_1);
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(month_0_ref_0);
    let mut month_1: month::Month = crate::month::Month::February;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2873() {
    rusty_monitor::set_test_id(2873);
    let mut i8_0: i8 = 80i8;
    let mut i8_1: i8 = 72i8;
    let mut i8_2: i8 = 1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut i8_3: i8 = -72i8;
    let mut i8_4: i8 = 49i8;
    let mut i8_5: i8 = -67i8;
    let mut i8_6: i8 = -50i8;
    let mut i8_7: i8 = 41i8;
    let mut i8_8: i8 = 10i8;
    let mut month_0: month::Month = crate::month::Month::October;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(month_0_ref_0);
    let mut month_1: month::Month = crate::month::Month::August;
    let mut i128_0: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3814() {
    rusty_monitor::set_test_id(3814);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut i64_0: i64 = -162i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut padding_0_ref_0: &duration::Padding = &mut padding_0;
    let mut i32_0: i32 = 61i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut month_1: month::Month = crate::month::Month::September;
    let mut i32_1: i32 = 59i32;
    let mut f32_0: f32 = -21.293240f32;
    let mut u8_0: u8 = 84u8;
    let mut u8_1: u8 = 34u8;
    let mut i128_0: i128 = -25i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut month_2: month::Month = crate::month::Month::August;
    let mut u8_2: u8 = crate::util::days_in_year_month(i32_1, month_1);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_0);
    let mut padding_1_ref_0: &duration::Padding = &mut padding_1;
    let mut tuple_0: (u8, u8, u8) = crate::offset_date_time::OffsetDateTime::to_hms(offsetdatetime_1);
    let mut month_2_ref_0: &month::Month = &mut month_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(month_2_ref_0, month_0_ref_0);
    panic!("From RustyUnit with love");
}
}