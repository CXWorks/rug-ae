//! Macros to construct statically known values.

/// Construct a [`Date`](crate::Date) with a statically known value.
///
/// The resulting expression can be used in `const` or `static` declarations.
///
/// Three formats are supported: year-week-weekday, year-ordinal, and year-month-day.
///
/// ```rust
/// # use time::{Date, Weekday::*, Month, macros::date};
/// assert_eq!(
///     date!(2020 - W 01 - 3),
///     Date::from_iso_week_date(2020, 1, Wednesday)?
/// );
/// assert_eq!(date!(2020 - 001), Date::from_ordinal_date(2020, 1)?);
/// assert_eq!(
///     date!(2020 - 01 - 01),
///     Date::from_calendar_date(2020, Month::January, 1)?
/// );
/// # Ok::<_, time::Error>(())
/// ```
pub use time_macros::date;
/// Construct a [`PrimitiveDateTime`] or [`OffsetDateTime`] with a statically known value.
///
/// The resulting expression can be used in `const` or `static` declarations.
///
/// The syntax accepted by this macro is the same as [`date!`] and [`time!`], with an optional
/// [`offset!`], all space-separated. If an [`offset!`] is provided, the resulting value will
/// be an [`OffsetDateTime`]; otherwise it will be a [`PrimitiveDateTime`].
///
/// [`OffsetDateTime`]: crate::OffsetDateTime
/// [`PrimitiveDateTime`]: crate::PrimitiveDateTime
pub use time_macros::datetime;
/// Equivalent of performing [`format_description::parse()`] at compile time.
///
/// Using the macro instead of the function results in a static slice rather than a [`Vec`],
/// such that it can be used in `#![no_alloc]` situations.
///
/// The resulting expression can be used in `const` or `static` declarations, and implements
/// the sealed traits required for both formatting and parsing.
///
/// ```rust
/// # use time::{format_description, macros::format_description};
/// assert_eq!(
///     format_description!("[hour]:[minute]:[second]"),
///     format_description::parse("[hour]:[minute]:[second]")?
/// );
/// # Ok::<_, time::Error>(())
/// ```
///
/// The syntax accepted by this macro is the same as [`format_description::parse()`], which can
/// be found in [the book](https://time-rs.github.io/book/api/format-description.html).
///
/// [`format_description::parse()`]: crate::format_description::parse()
#[cfg(any(feature = "formatting", feature = "parsing"))]
pub use time_macros::format_description;
/// Construct a [`UtcOffset`](crate::UtcOffset) with a statically known value.
///
/// The resulting expression can be used in `const` or `static` declarations.
///
/// A sign and the hour must be provided; minutes and seconds default to zero. `UTC` (both
/// uppercase and lowercase) is also allowed.
///
/// ```rust
/// # use time::{UtcOffset, macros::offset};
/// assert_eq!(offset!(UTC), UtcOffset::from_hms(0, 0, 0)?);
/// assert_eq!(offset!(utc), UtcOffset::from_hms(0, 0, 0)?);
/// assert_eq!(offset!(+0), UtcOffset::from_hms(0, 0, 0)?);
/// assert_eq!(offset!(+1), UtcOffset::from_hms(1, 0, 0)?);
/// assert_eq!(offset!(-1), UtcOffset::from_hms(-1, 0, 0)?);
/// assert_eq!(offset!(+1:30), UtcOffset::from_hms(1, 30, 0)?);
/// assert_eq!(offset!(-1:30), UtcOffset::from_hms(-1, -30, 0)?);
/// assert_eq!(offset!(+1:30:59), UtcOffset::from_hms(1, 30, 59)?);
/// assert_eq!(offset!(-1:30:59), UtcOffset::from_hms(-1, -30, -59)?);
/// assert_eq!(offset!(+23:59:59), UtcOffset::from_hms(23, 59, 59)?);
/// assert_eq!(offset!(-23:59:59), UtcOffset::from_hms(-23, -59, -59)?);
/// # Ok::<_, time::Error>(())
/// ```
pub use time_macros::offset;
/// Construct a [`Time`](crate::Time) with a statically known value.
///
/// The resulting expression can be used in `const` or `static` declarations.
///
/// Hours and minutes must be provided, while seconds defaults to zero. AM/PM is allowed
/// (either uppercase or lowercase). Any number of subsecond digits may be provided (though any
/// past nine will be discarded).
///
/// All components are validated at compile-time. An error will be raised if any value is
/// invalid.
///
/// ```rust
/// # use time::{Time, macros::time};
/// assert_eq!(time!(0:00), Time::from_hms(0, 0, 0)?);
/// assert_eq!(time!(1:02:03), Time::from_hms(1, 2, 3)?);
/// assert_eq!(
///     time!(1:02:03.004_005_006),
///     Time::from_hms_nano(1, 2, 3, 4_005_006)?
/// );
/// assert_eq!(time!(12:00 am), Time::from_hms(0, 0, 0)?);
/// assert_eq!(time!(1:02:03 am), Time::from_hms(1, 2, 3)?);
/// assert_eq!(
///     time!(1:02:03.004_005_006 am),
///     Time::from_hms_nano(1, 2, 3, 4_005_006)?
/// );
/// assert_eq!(time!(12 pm), Time::from_hms(12, 0, 0)?);
/// assert_eq!(time!(12:00 pm), Time::from_hms(12, 0, 0)?);
/// assert_eq!(time!(1:02:03 pm), Time::from_hms(13, 2, 3)?);
/// assert_eq!(
///     time!(1:02:03.004_005_006 pm),
///     Time::from_hms_nano(13, 2, 3, 4_005_006)?
/// );
/// # Ok::<_, time::Error>(())
/// ```
pub use time_macros::time;

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_625() {
//    rusty_monitor::set_test_id(625);
    let mut i8_0: i8 = 3i8;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = 0i8;
    let mut i8_3: i8 = 6i8;
    let mut i8_4: i8 = 6i8;
    let mut i8_5: i8 = 59i8;
    let mut i8_6: i8 = 59i8;
    let mut i8_7: i8 = 6i8;
    let mut i8_8: i8 = 60i8;
    let mut i8_9: i8 = 24i8;
    let mut i8_10: i8 = 23i8;
    let mut i8_11: i8 = 2i8;
    let mut i8_12: i8 = 2i8;
    let mut i8_13: i8 = 60i8;
    let mut i8_14: i8 = 24i8;
    let mut i8_15: i8 = 2i8;
    let mut i8_16: i8 = 3i8;
    let mut i8_17: i8 = 0i8;
    let mut i8_18: i8 = 3i8;
    let mut i8_19: i8 = 23i8;
    let mut i8_20: i8 = 0i8;
    let mut i8_21: i8 = 60i8;
    let mut i8_22: i8 = 0i8;
    let mut i8_23: i8 = 24i8;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_23, i8_22, i8_21);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_20, i8_19, i8_18);
    let mut result_2: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_17, i8_16, i8_15);
    let mut result_3: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_14, i8_13, i8_12);
    let mut result_4: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_11, i8_10, i8_9);
    let mut result_5: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_8, i8_7, i8_6);
    let mut result_6: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_5, i8_4, i8_3);
    let mut result_7: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_488() {
//    rusty_monitor::set_test_id(488);
    let mut i32_0: i32 = 2147483647i32;
    let mut i32_1: i32 = 381i32;
    let mut i32_2: i32 = 1721425i32;
    let mut i32_3: i32 = 280i32;
    let mut i32_4: i32 = 2i32;
    let mut i32_5: i32 = 308i32;
    let mut i32_6: i32 = 82i32;
    let mut i32_7: i32 = 263i32;
    let mut i32_8: i32 = 76i32;
    let mut i32_9: i32 = 105i32;
    let mut i32_10: i32 = 224i32;
    let mut i32_11: i32 = 184i32;
    let mut i32_12: i32 = 105i32;
    let mut i32_13: i32 = 184i32;
    let mut i32_14: i32 = 229i32;
    let mut i32_15: i32 = 331i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_15);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_14);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_13);
    let mut result_3: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_12);
    let mut result_4: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_11);
    let mut result_5: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_10);
    let mut result_6: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_9);
    let mut result_7: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_8);
    let mut result_8: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_7);
    let mut result_9: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_6);
    let mut result_10: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_5);
    let mut result_11: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_4);
    let mut result_12: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_3);
    let mut result_13: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_2);
    let mut result_14: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_1);
    let mut result_15: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_julian_day(i32_0);
//    panic!("From RustyUnit with love");
}
}