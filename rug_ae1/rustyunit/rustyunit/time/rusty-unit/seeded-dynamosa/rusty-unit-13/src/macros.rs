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
#[timeout(30000)]fn rusty_test_6578() {
//    rusty_monitor::set_test_id(6578);
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut i64_0: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = 23i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_1: i64 = 12i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 10u8;
    let mut u8_1: u8 = 58u8;
    let mut u8_2: u8 = 62u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 36525i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut i32_1: i32 = 99i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 14i32;
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut i32_3: i32 = 342i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i8_3: i8 = 126i8;
    let mut i8_4: i8 = 5i8;
    let mut i8_5: i8 = 5i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_4: i32 = 189i32;
    let mut i64_3: i64 = 86400i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_4);
    let mut i128_0: i128 = 1000i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 10000u32;
    let mut u8_3: u8 = 52u8;
    let mut u8_4: u8 = 5u8;
    let mut u8_5: u8 = 7u8;
    let mut f64_1: f64 = 0.000000f64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut u16_0: u16 = 999u16;
    let mut i32_5: i32 = 511i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_0);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_9);
    let mut i32_6: i32 = 240i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_5);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_4: i64 = 2147483647i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::days(i64_4);
    let mut u16_1: u16 = 365u16;
    let mut i32_7: i32 = 116i32;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_1);
    let mut u32_2: u32 = 1000u32;
    let mut u8_6: u8 = 12u8;
    let mut u8_7: u8 = 3u8;
    let mut u8_8: u8 = 60u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i32_8: i32 = 398i32;
    let mut date_7: crate::date::Date = crate::date::Date {value: i32_8};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_7, time_1);
    let mut bool_0: bool = true;
    let mut i64_5: i64 = 2147483647i64;
    let mut i64_6: i64 = 24i64;
    let mut i64_7: i64 = 2440588i64;
    let mut str_0: &str = "November";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_7, maximum: i64_6, value: i64_5, conditional_range: bool_0};
    let mut componentrange_0_ref_0: &crate::error::component_range::ComponentRange = &mut componentrange_0;
    let mut bool_1: bool = false;
    let mut i64_8: i64 = 60i64;
    let mut i64_9: i64 = 232i64;
    let mut i64_10: i64 = 1000i64;
    let mut str_1: &str = "minutes";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_10, maximum: i64_9, value: i64_8, conditional_range: bool_1};
//    panic!("From RustyUnit with love");
}
}