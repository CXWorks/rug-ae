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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4776() {
    rusty_monitor::set_test_id(4776);
    let mut u16_0: u16 = 80u16;
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_0: u32 = 12u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 25u8;
    let mut u8_2: u8 = 53u8;
    let mut i32_0: i32 = 99i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i32_1: i32 = 61i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut u16_1: u16 = 29u16;
    let mut i32_2: i32 = -56i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut i32_3: i32 = 46i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_2);
    let mut i8_0: i8 = 85i8;
    let mut i8_1: i8 = 8i8;
    let mut i8_2: i8 = 62i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_4: i32 = -42i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_4, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_1, primitivedatetime_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut u16_2: u16 = 37u16;
    let mut i32_5: i32 = -29i32;
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_2);
    let mut u16_3: u16 = 9u16;
    let mut i32_6: i32 = -91i32;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_6, u16_3);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_6, date_5);
    let mut time_2: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_7);
    let mut month_0: month::Month = crate::month::Month::January;
    let mut month_1: month::Month = crate::month::Month::August;
    panic!("From RustyUnit with love");
}
}