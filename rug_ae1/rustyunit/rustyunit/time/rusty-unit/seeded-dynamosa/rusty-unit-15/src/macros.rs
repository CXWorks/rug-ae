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
#[timeout(30000)]fn rusty_test_8026() {
//    rusty_monitor::set_test_id(8026);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut u8_0: u8 = 11u8;
    let mut i32_0: i32 = -41i32;
    let mut i32_1: i32 = 387i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut i64_0: i64 = 9223372036854775807i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_2: i32 = 257i32;
    let mut i32_3: i32 = 398i32;
    let mut i64_1: i64 = 9223372036854775807i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_2);
    let mut u32_0: u32 = 10u32;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 10u8;
    let mut u8_3: u8 = 24u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_4, time_0);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Previous;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_0, u8_0, weekday_0);
//    panic!("From RustyUnit with love");
}
}