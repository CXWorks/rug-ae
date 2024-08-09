//! Utility functions.

use crate::Month;

/// Whether to adjust the date, and in which direction. Useful when implementing arithmetic.
pub(crate) enum DateAdjustment {
    /// The previous day should be used.
    Previous,
    /// The next day should be used.
    Next,
    /// The date should be used as-is.
    None,
}

/// Get the number of days in the month of a given year.
///
/// ```rust
/// # use time::{Month, util};
/// assert_eq!(util::days_in_year_month(2020, Month::February), 29);
/// ```
pub const fn days_in_year_month(year: i32, month: Month) -> u8 {
    use Month::*;
    match month {
        January | March | May | July | August | October | December => 31,
        April | June | September | November => 30,
        February if is_leap_year(year) => 29,
        February => 28,
    }
}

/// Returns if the provided year is a leap year in the proleptic Gregorian calendar. Uses
/// [astronomical year numbering](https://en.wikipedia.org/wiki/Astronomical_year_numbering).
///
/// ```rust
/// # use time::util::is_leap_year;
/// assert!(!is_leap_year(1900));
/// assert!(is_leap_year(2000));
/// assert!(is_leap_year(2004));
/// assert!(!is_leap_year(2005));
/// assert!(!is_leap_year(2100));
/// ```
pub const fn is_leap_year(year: i32) -> bool {
    year % 4 == 0 && (year % 25 != 0 || year % 16 == 0)
}

/// Get the number of calendar days in a given year.
///
/// The returned value will always be either 365 or 366.
///
/// ```rust
/// # use time::util::days_in_year;
/// assert_eq!(days_in_year(1900), 365);
/// assert_eq!(days_in_year(2000), 366);
/// assert_eq!(days_in_year(2004), 366);
/// assert_eq!(days_in_year(2005), 365);
/// assert_eq!(days_in_year(2100), 365);
/// ```
pub const fn days_in_year(year: i32) -> u16 {
    if is_leap_year(year) { 366 } else { 365 }
}

/// Get the number of weeks in the ISO year.
///
/// The returned value will always be either 52 or 53.
///
/// ```rust
/// # use time::util::weeks_in_year;
/// assert_eq!(weeks_in_year(2019), 52);
/// assert_eq!(weeks_in_year(2020), 53);
/// ```
pub const fn weeks_in_year(year: i32) -> u8 {
    match year.rem_euclid(400) {
        4 | 9 | 15 | 20 | 26 | 32 | 37 | 43 | 48 | 54 | 60 | 65 | 71 | 76 | 82 | 88 | 93 | 99
        | 105 | 111 | 116 | 122 | 128 | 133 | 139 | 144 | 150 | 156 | 161 | 167 | 172 | 178
        | 184 | 189 | 195 | 201 | 207 | 212 | 218 | 224 | 229 | 235 | 240 | 246 | 252 | 257
        | 263 | 268 | 274 | 280 | 285 | 291 | 296 | 303 | 308 | 314 | 320 | 325 | 331 | 336
        | 342 | 348 | 353 | 359 | 364 | 370 | 376 | 381 | 387 | 392 | 398 => 53,
        _ => 52,
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6573() {
    rusty_monitor::set_test_id(6573);
    let mut i64_0: i64 = -122i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut u32_0: u32 = 42u32;
    let mut u8_0: u8 = 17u8;
    let mut u8_1: u8 = 33u8;
    let mut u8_2: u8 = 34u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 13i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut i32_0: i32 = 119i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u32_1: u32 = 93u32;
    let mut u8_3: u8 = 63u8;
    let mut u8_4: u8 = 70u8;
    let mut u8_5: u8 = 80u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = -1i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_1};
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut i32_2: i32 = 68i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut u16_0: u16 = 56u16;
    let mut i32_3: i32 = -37i32;
    let mut i32_4: i32 = 66i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut i32_5: i32 = 84i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_4);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_1: u16 = 79u16;
    let mut i32_6: i32 = -217i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_5, time_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_3);
    let mut i32_7: i32 = 121i32;
    let mut i64_2: i64 = -96i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_7);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = 39i8;
    let mut i8_2: i8 = 120i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_8: i32 = -95i32;
    let mut date_6: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_8);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_2);
    let mut time_3: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i8_3: i8 = -34i8;
    let mut i8_4: i8 = 33i8;
    let mut i8_5: i8 = 95i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 66i8;
    let mut i8_7: i8 = -41i8;
    let mut i8_8: i8 = 20i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_3: i64 = -4i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut u32_2: u32 = 2u32;
    let mut u8_6: u8 = 68u8;
    let mut u8_7: u8 = 27u8;
    let mut u8_8: u8 = 87u8;
    let mut u8_9: u8 = 67u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_6, u8_8, u8_7, u32_2);
    let mut i32_9: i32 = 61i32;
    let mut date_7: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_3};
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_9);
    let mut i32_10: i32 = crate::primitive_date_time::PrimitiveDateTime::year(primitivedatetime_2);
    let mut date_8: crate::date::Date = crate::date::Date::saturating_sub(date_7, duration_3);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_10, u8_9, weekday_0);
    let mut tuple_0: (i32, u16) = crate::primitive_date_time::PrimitiveDateTime::to_ordinal_date(primitivedatetime_4);
    panic!("From RustyUnit with love");
}
}