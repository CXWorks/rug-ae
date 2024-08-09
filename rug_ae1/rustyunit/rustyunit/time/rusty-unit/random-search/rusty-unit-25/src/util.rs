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
fn rusty_test_4153() {
    rusty_monitor::set_test_id(4153);
    let mut u16_0: u16 = 11u16;
    let mut i32_0: i32 = -168i32;
    let mut i16_0: i16 = -54i16;
    let mut i64_0: i64 = 17i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i32_1: i32 = 316i32;
    let mut i32_2: i32 = -130i32;
    let mut f32_0: f32 = -32.386606f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_3: i32 = -153i32;
    let mut i64_1: i64 = -57i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut padding_0: duration::Padding = crate::duration::Padding::Optimize;
    let mut i32_4: i32 = -97i32;
    let mut i64_2: i64 = -72i64;
    let mut i32_5: i32 = 56i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut u8_0: u8 = crate::util::weeks_in_year(i32_1);
    let mut u16_1: u16 = crate::util::days_in_year(i32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4974() {
    rusty_monitor::set_test_id(4974);
    let mut f32_0: f32 = 13.466666f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i32_0: i32 = -24i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 7u8;
    let mut u8_1: u8 = 87u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = 161i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut u16_0: u16 = 17u16;
    let mut i32_1: i32 = -16i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_3, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut i8_0: i8 = 97i8;
    let mut i8_1: i8 = 61i8;
    let mut i8_2: i8 = -18i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_1: u32 = 67u32;
    let mut u8_3: u8 = 71u8;
    let mut u8_4: u8 = 51u8;
    let mut u8_5: u8 = 0u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_1: i64 = 62i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut i64_2: i64 = -154i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut u16_1: u16 = 92u16;
    let mut i32_2: i32 = 72i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_5, time_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_2, offset: utcoffset_0};
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut u8_6: u8 = crate::offset_date_time::OffsetDateTime::minute(offsetdatetime_1);
    panic!("From RustyUnit with love");
}
}