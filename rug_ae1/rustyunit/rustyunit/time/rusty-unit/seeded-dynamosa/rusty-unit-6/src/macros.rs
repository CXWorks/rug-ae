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
#[timeout(30000)]fn rusty_test_8798() {
//    rusty_monitor::set_test_id(8798);
    let mut u16_0: u16 = 10u16;
    let mut i32_0: i32 = 184i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u8_0: u8 = 59u8;
    let mut i32_1: i32 = -47i32;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut u8_1: u8 = 23u8;
    let mut i32_2: i32 = 280i32;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut u8_2: u8 = 5u8;
    let mut i32_3: i32 = 285i32;
    let mut i32_4: i32 = -54i32;
    let mut i64_0: i64 = 1000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_4);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_3, u8_2, weekday_3);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_2, u8_1, weekday_2);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_0, weekday_1);
//    panic!("From RustyUnit with love");
}
}