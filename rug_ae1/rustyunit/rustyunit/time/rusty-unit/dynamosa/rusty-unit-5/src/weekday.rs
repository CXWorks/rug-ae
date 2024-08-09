//! Days of the week.

use core::fmt::{self, Display};

use Weekday::*;

/// Days of the week.
///
/// As order is dependent on context (Sunday could be either two days after or five days before
/// Friday), this type does not implement `PartialOrd` or `Ord`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Weekday {
    #[allow(clippy::missing_docs_in_private_items)]
    Monday,
    #[allow(clippy::missing_docs_in_private_items)]
    Tuesday,
    #[allow(clippy::missing_docs_in_private_items)]
    Wednesday,
    #[allow(clippy::missing_docs_in_private_items)]
    Thursday,
    #[allow(clippy::missing_docs_in_private_items)]
    Friday,
    #[allow(clippy::missing_docs_in_private_items)]
    Saturday,
    #[allow(clippy::missing_docs_in_private_items)]
    Sunday,
}

impl Weekday {
    /// Get the previous weekday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Tuesday.previous(), Weekday::Monday);
    /// ```
    pub const fn previous(self) -> Self {
        match self {
            Monday => Sunday,
            Tuesday => Monday,
            Wednesday => Tuesday,
            Thursday => Wednesday,
            Friday => Thursday,
            Saturday => Friday,
            Sunday => Saturday,
        }
    }

    /// Get the next weekday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.next(), Weekday::Tuesday);
    /// ```
    pub const fn next(self) -> Self {
        match self {
            Monday => Tuesday,
            Tuesday => Wednesday,
            Wednesday => Thursday,
            Thursday => Friday,
            Friday => Saturday,
            Saturday => Sunday,
            Sunday => Monday,
        }
    }

    /// Get the one-indexed number of days from Monday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_from_monday(), 1);
    /// ```
    #[doc(alias = "iso_weekday_number")]
    pub const fn number_from_monday(self) -> u8 {
        self.number_days_from_monday() + 1
    }

    /// Get the one-indexed number of days from Sunday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_from_sunday(), 2);
    /// ```
    pub const fn number_from_sunday(self) -> u8 {
        self.number_days_from_sunday() + 1
    }

    /// Get the zero-indexed number of days from Monday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_days_from_monday(), 0);
    /// ```
    pub const fn number_days_from_monday(self) -> u8 {
        self as _
    }

    /// Get the zero-indexed number of days from Sunday.
    ///
    /// ```rust
    /// # use time::Weekday;
    /// assert_eq!(Weekday::Monday.number_days_from_sunday(), 1);
    /// ```
    pub const fn number_days_from_sunday(self) -> u8 {
        match self {
            Monday => 1,
            Tuesday => 2,
            Wednesday => 3,
            Thursday => 4,
            Friday => 5,
            Saturday => 6,
            Sunday => 0,
        }
    }
}

impl Display for Weekday {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Monday => "Monday",
            Tuesday => "Tuesday",
            Wednesday => "Wednesday",
            Thursday => "Thursday",
            Friday => "Friday",
            Saturday => "Saturday",
            Sunday => "Sunday",
        })
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::clone::Clone;
	use std::cmp::PartialEq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7635() {
    rusty_monitor::set_test_id(7635);
    let mut i64_0: i64 = -159i64;
    let mut i64_1: i64 = 57i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut i8_0: i8 = 27i8;
    let mut i8_1: i8 = -34i8;
    let mut i8_2: i8 = 67i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 45i32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut u32_0: u32 = 51u32;
    let mut u8_0: u8 = 83u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 17u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -8i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_2: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6689() {
    rusty_monitor::set_test_id(6689);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut f64_0: f64 = 157.433394f64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut u16_0: u16 = 0u16;
    let mut i32_0: i32 = 61i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_2);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut u8_0: u8 = 7u8;
    let mut i32_1: i32 = 71i32;
    let mut u16_1: u16 = 37u16;
    let mut i32_2: i32 = -124i32;
    let mut i32_3: i32 = 62i32;
    let mut i64_0: i64 = 59i64;
    let mut u32_0: u32 = 89u32;
    let mut u8_1: u8 = 15u8;
    let mut u8_2: u8 = 99u8;
    let mut u8_3: u8 = 76u8;
    let mut i32_4: i32 = -27i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_4};
    let mut i32_5: i32 = 188i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_5};
    let mut i32_6: i32 = 54i32;
    let mut i64_1: i64 = -109i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_6);
    let mut i64_2: i64 = 126i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_7: i32 = 13i32;
    let mut i64_3: i64 = -52i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_7);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut f64_1: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_1: u32 = 36u32;
    let mut u8_4: u8 = 59u8;
    let mut u8_5: u8 = 97u8;
    let mut u8_6: u8 = 70u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_6, u8_5, u8_4, u32_1);
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut weekday_3: weekday::Weekday = std::clone::Clone::clone(weekday_1_ref_0);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_4);
    let mut i32_8: i32 = crate::duration::Duration::subsec_nanoseconds(duration_3);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_3);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_2, u8_3, u8_2, u8_1, u32_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_3);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_2, u16_1);
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut result_3: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_0, weekday_0);
    let mut u16_2: u16 = crate::date::Date::ordinal(date_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2498() {
    rusty_monitor::set_test_id(2498);
    let mut i64_0: i64 = -98i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut u16_0: u16 = 82u16;
    let mut i32_0: i32 = 16i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_2);
    let mut i32_1: i32 = 41i32;
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = -65i8;
    let mut i8_2: i8 = -62i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_3, utcoffset_0);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut i8_3: i8 = -110i8;
    let mut i8_4: i8 = -107i8;
    let mut i8_5: i8 = 72i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 38i8;
    let mut i8_7: i8 = 75i8;
    let mut i8_8: i8 = 55i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_1: i64 = 27i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_3: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7090() {
    rusty_monitor::set_test_id(7090);
    let mut i8_0: i8 = 98i8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut i32_0: i32 = 64i32;
    let mut i64_0: i64 = -56i64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut i64_1: i64 = -69i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i8_1: i8 = 23i8;
    let mut i8_2: i8 = -3i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_0, i8_1, i8_2);
    let mut f32_0: f32 = 31.437427f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut f64_0: f64 = -117.912420f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_1: f64 = 169.057512f64;
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut weekday_3: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_185() {
    rusty_monitor::set_test_id(185);
    let mut i8_0: i8 = 12i8;
    let mut i8_1: i8 = 13i8;
    let mut i8_2: i8 = 46i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 188i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i32_1: i32 = 54i32;
    let mut i64_0: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_1);
    let mut i64_1: i64 = 126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_2: i32 = 13i32;
    let mut i64_2: i64 = -52i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_0: u32 = 36u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 97u8;
    let mut u8_2: u8 = 70u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_2, u8_1, u8_0, u32_0);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_2: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    let mut i32_3: i32 = crate::duration::Duration::subsec_nanoseconds(duration_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_utc(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4816() {
    rusty_monitor::set_test_id(4816);
    let mut u8_0: u8 = 19u8;
    let mut month_0: month::Month = crate::month::Month::May;
    let mut i32_0: i32 = 56i32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut i64_0: i64 = -77i64;
    let mut i32_1: i32 = 95i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 188i32;
    let mut i64_1: i64 = -109i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut i64_2: i64 = 126i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_3: i32 = 13i32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_3: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_0, u8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_112() {
    rusty_monitor::set_test_id(112);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut i32_0: i32 = -267i32;
    let mut i64_0: i64 = -70i64;
    let mut i64_1: i64 = 72i64;
    let mut i32_1: i32 = -11i32;
    let mut i32_2: i32 = 112i32;
    let mut i64_2: i64 = 56i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_0, i32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_3: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_473() {
    rusty_monitor::set_test_id(473);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut i64_0: i64 = 16i64;
    let mut i32_0: i32 = 198i32;
    let mut i64_1: i64 = -135i64;
    let mut i64_2: i64 = 207i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i32_1: i32 = 36i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i64_3: i64 = 126i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_2: i32 = 13i32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_2);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_4: weekday::Weekday = std::clone::Clone::clone(weekday_1_ref_0);
    let mut duration_1_ref_0: &mut crate::duration::Duration = &mut duration_1;
    let mut i64_4: i64 = crate::duration::Duration::whole_weeks(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3996() {
    rusty_monitor::set_test_id(3996);
    let mut i64_0: i64 = 10i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_1: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut i32_0: i32 = -66i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut weekday_1: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
    let mut i32_1: i32 = 62i32;
    let mut i64_1: i64 = 59i64;
    let mut u32_0: u32 = 89u32;
    let mut u8_0: u8 = 15u8;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 76u8;
    let mut i32_2: i32 = -27i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = 188i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i32_4: i32 = 54i32;
    let mut i64_2: i64 = -109i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_4);
    let mut i64_3: i64 = 126i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_5: i32 = 13i32;
    let mut i64_4: i64 = -52i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_5);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_3_ref_0: &weekday::Weekday = &mut weekday_3;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_1: u32 = 36u32;
    let mut u8_3: u8 = 59u8;
    let mut u8_4: u8 = 97u8;
    let mut u8_5: u8 = 70u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_5, u8_4, u8_3, u32_1);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_5: weekday::Weekday = std::clone::Clone::clone(weekday_3_ref_0);
    let mut duration_6_ref_0: &mut crate::duration::Duration = &mut duration_6;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_4);
    let mut i32_6: i32 = crate::duration::Duration::subsec_nanoseconds(duration_3);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_1, u8_2, u8_1, u8_0, u32_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut bool_0: bool = std::cmp::PartialEq::eq(weekday_2_ref_0, weekday_0_ref_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4808() {
    rusty_monitor::set_test_id(4808);
    let mut u32_0: u32 = 11u32;
    let mut u8_0: u8 = 14u8;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 41u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_0, time_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut u32_1: u32 = 98u32;
    let mut u8_3: u8 = 3u8;
    let mut u8_4: u8 = 78u8;
    let mut u8_5: u8 = 27u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 71u32;
    let mut u8_6: u8 = 16u8;
    let mut u8_7: u8 = 87u8;
    let mut u8_8: u8 = 34u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_0, time_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i32_0: i32 = -9i32;
    let mut i64_0: i64 = 47i64;
    let mut i32_1: i32 = 62i32;
    let mut i64_1: i64 = 59i64;
    let mut u32_3: u32 = 89u32;
    let mut u8_9: u8 = 15u8;
    let mut u8_10: u8 = 99u8;
    let mut u8_11: u8 = 76u8;
    let mut i32_2: i32 = -27i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = 188i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i32_4: i32 = 54i32;
    let mut i64_2: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_4);
    let mut i64_3: i64 = 126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_4: u32 = 36u32;
    let mut u8_12: u8 = 59u8;
    let mut u8_13: u8 = 97u8;
    let mut u8_14: u8 = 70u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_14, u8_13, u8_12, u32_4);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_2: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    let mut i32_5: i32 = crate::duration::Duration::subsec_nanoseconds(duration_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_1, u8_11, u8_10, u8_9, u32_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut month_2: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_4);
    let mut u8_15: u8 = crate::primitive_date_time::PrimitiveDateTime::minute(primitivedatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1951() {
    rusty_monitor::set_test_id(1951);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = -65i8;
    let mut i8_2: i8 = -62i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = -110i8;
    let mut i8_4: i8 = -107i8;
    let mut i8_5: i8 = 72i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 38i8;
    let mut i8_7: i8 = 75i8;
    let mut i8_8: i8 = 55i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_0: i64 = 27i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_4: weekday::Weekday = std::clone::Clone::clone(weekday_1_ref_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2817() {
    rusty_monitor::set_test_id(2817);
    let mut u32_0: u32 = 79u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 84u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 33u16;
    let mut i32_0: i32 = 2i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut i32_1: i32 = 62i32;
    let mut i64_0: i64 = 59i64;
    let mut u32_1: u32 = 89u32;
    let mut u8_3: u8 = 15u8;
    let mut u8_4: u8 = 99u8;
    let mut u8_5: u8 = 76u8;
    let mut i32_2: i32 = -27i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = 198i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i32_4: i32 = 54i32;
    let mut i64_1: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_4);
    let mut i64_2: i64 = 126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_5: i32 = 13i32;
    let mut i64_3: i64 = -52i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_5);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_2: u32 = 36u32;
    let mut u8_6: u8 = 59u8;
    let mut u8_7: u8 = 97u8;
    let mut u8_8: u8 = 70u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_8, u8_7, u8_6, u32_2);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_2: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    let mut i32_6: i32 = crate::duration::Duration::subsec_nanoseconds(duration_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_1, u8_5, u8_4, u8_3, u32_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut month_2: month::Month = crate::month::Month::November;
    let mut tuple_1: (u8, u8, u8) = crate::primitive_date_time::PrimitiveDateTime::as_hms(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7287() {
    rusty_monitor::set_test_id(7287);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut i32_0: i32 = 62i32;
    let mut i64_0: i64 = 59i64;
    let mut u8_0: u8 = 99u8;
    let mut u8_1: u8 = 76u8;
    let mut i32_1: i32 = -27i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 188i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = 54i32;
    let mut i64_1: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut i64_2: i64 = 126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_0: u32 = 36u32;
    let mut u8_2: u8 = 59u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_2, u8_0, u8_1, u32_0);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_3: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6658() {
    rusty_monitor::set_test_id(6658);
    let mut month_0: month::Month = crate::month::Month::November;
    let mut month_1: month::Month = crate::month::Month::next(month_0);
    let mut i32_0: i32 = -38i32;
    let mut i8_0: i8 = -8i8;
    let mut i8_1: i8 = -9i8;
    let mut i8_2: i8 = 7i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = -32i8;
    let mut i8_4: i8 = 88i8;
    let mut i8_5: i8 = -80i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_0: i64 = 49i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i8_6: i8 = -65i8;
    let mut i8_7: i8 = -66i8;
    let mut i8_8: i8 = -3i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_1: i64 = -230i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i8_9: i8 = 96i8;
    let mut i8_10: i8 = -66i8;
    let mut i8_11: i8 = -62i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut u32_0: u32 = 53u32;
    let mut u8_0: u8 = 82u8;
    let mut u8_1: u8 = 29u8;
    let mut u8_2: u8 = 85u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u16_0: u16 = 37u16;
    let mut i32_1: i32 = -124i32;
    let mut i32_2: i32 = 62i32;
    let mut i64_2: i64 = 59i64;
    let mut u32_1: u32 = 89u32;
    let mut u8_3: u8 = 15u8;
    let mut u8_4: u8 = 99u8;
    let mut u8_5: u8 = 76u8;
    let mut i32_3: i32 = -27i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i32_4: i32 = 188i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_4};
    let mut i32_5: i32 = 54i32;
    let mut i64_3: i64 = -109i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_5);
    let mut i64_4: i64 = 126i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_6: i32 = 13i32;
    let mut i64_5: i64 = -52i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_6);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_2: u32 = 36u32;
    let mut u8_6: u8 = 59u8;
    let mut u8_7: u8 = 97u8;
    let mut u8_8: u8 = 70u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_8, u8_7, u8_6, u32_2);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_2: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_3);
    let mut i32_7: i32 = crate::duration::Duration::subsec_nanoseconds(duration_2);
    let mut month_2: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_3: month::Month = crate::month::Month::June;
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_1, u8_5, u8_4, u8_3, u32_1);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_1, u16_0);
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut u8_9: u8 = crate::date::Date::iso_week(date_0);
    let mut u8_10: u8 = crate::util::days_in_year_month(i32_0, month_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7817() {
    rusty_monitor::set_test_id(7817);
    let mut i32_0: i32 = 0i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut i32_1: i32 = 6i32;
    let mut i64_0: i64 = -224i64;
    let mut f64_0: f64 = 112.821219f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_2: i32 = 35i32;
    let mut i64_1: i64 = 120i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut i32_3: i32 = 151i32;
    let mut i64_2: i64 = 86i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_3);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut f64_1: f64 = -6.278623f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_3: i64 = -18i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut i64_4: i64 = 11i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i64_5: i64 = 3i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_2: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut weekday_3: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2592() {
    rusty_monitor::set_test_id(2592);
    let mut i32_0: i32 = 77i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut i32_1: i32 = -182i32;
    let mut i64_0: i64 = 59i64;
    let mut i32_2: i32 = -27i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = 54i32;
    let mut i64_1: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_3);
    let mut i64_2: i64 = 126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_4: i32 = 13i32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_4);
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_2: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6044() {
    rusty_monitor::set_test_id(6044);
    let mut u32_0: u32 = 38u32;
    let mut u8_0: u8 = 68u8;
    let mut u8_1: u8 = 64u8;
    let mut u8_2: u8 = 79u8;
    let mut i32_0: i32 = -37i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u8_3: u8 = 67u8;
    let mut u8_4: u8 = 78u8;
    let mut u8_5: u8 = 14u8;
    let mut u16_0: u16 = 37u16;
    let mut i32_1: i32 = -124i32;
    let mut i32_2: i32 = 62i32;
    let mut i64_0: i64 = 59i64;
    let mut u32_1: u32 = 89u32;
    let mut u8_6: u8 = 15u8;
    let mut u8_7: u8 = 99u8;
    let mut u8_8: u8 = 76u8;
    let mut i32_3: i32 = -27i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i32_4: i32 = 188i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_4};
    let mut i32_5: i32 = 54i32;
    let mut i64_1: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_5);
    let mut i64_2: i64 = 126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_6: i32 = 13i32;
    let mut i64_3: i64 = -52i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_6);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_2: u32 = 36u32;
    let mut u8_9: u8 = 59u8;
    let mut u8_10: u8 = 97u8;
    let mut u8_11: u8 = 70u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_11, u8_10, u8_9, u32_2);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_2: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    let mut i32_7: i32 = crate::duration::Duration::subsec_nanoseconds(duration_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_1, u8_8, u8_7, u8_6, u32_1);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_2);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_1, u16_0);
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut result_3: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_5, u8_4, u8_3);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut result_4: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_0, u8_2, u8_1, u8_0, u32_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_472() {
    rusty_monitor::set_test_id(472);
    let mut i32_0: i32 = 188i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i64_0: i64 = 126i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_1: i32 = 13i32;
    let mut i64_1: i64 = -52i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_0: u32 = 36u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 97u8;
    let mut u8_2: u8 = 70u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_2, u8_1, u8_0, u32_0);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_2: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Friday;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_550() {
    rusty_monitor::set_test_id(550);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut u8_0: u8 = 71u8;
    let mut i32_0: i32 = 51i32;
    let mut i32_1: i32 = 188i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 54i32;
    let mut i64_0: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut i64_1: i64 = 126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_3: i32 = 13i32;
    let mut i64_2: i64 = -52i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_0: u32 = 36u32;
    let mut u8_1: u8 = 59u8;
    let mut u8_2: u8 = 97u8;
    let mut u8_3: u8 = 70u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_3, u8_2, u8_1, u32_0);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_3: weekday::Weekday = std::clone::Clone::clone(weekday_1_ref_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    let mut i32_4: i32 = crate::duration::Duration::subsec_nanoseconds(duration_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_0, u8_0, weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7151() {
    rusty_monitor::set_test_id(7151);
    let mut i64_0: i64 = -128i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i64_1: i64 = -116i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut u16_0: u16 = 37u16;
    let mut i32_0: i32 = -124i32;
    let mut i32_1: i32 = 62i32;
    let mut i64_2: i64 = 59i64;
    let mut u32_0: u32 = 89u32;
    let mut u8_0: u8 = 15u8;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 76u8;
    let mut i32_2: i32 = -27i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = 188i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i32_4: i32 = 54i32;
    let mut i64_3: i64 = -109i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_4);
    let mut i64_4: i64 = 126i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_5: i32 = 13i32;
    let mut i64_5: i64 = -52i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_5, i32_5);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_2: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_5_ref_0: &mut crate::duration::Duration = &mut duration_5;
    let mut i32_6: i32 = crate::duration::Duration::subsec_nanoseconds(duration_2);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_0, u16_0);
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut f64_1: f64 = crate::duration::Duration::as_seconds_f64(duration_1);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5083() {
    rusty_monitor::set_test_id(5083);
    let mut i32_0: i32 = 134i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut i64_0: i64 = 127i64;
    let mut i8_0: i8 = 36i8;
    let mut i8_1: i8 = -93i8;
    let mut i8_2: i8 = -73i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 201i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut u32_0: u32 = 89u32;
    let mut u8_0: u8 = 15u8;
    let mut u8_1: u8 = 42u8;
    let mut u8_2: u8 = 21u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_4: weekday::Weekday = std::clone::Clone::clone(weekday_1_ref_0);
    let mut duration_0_ref_0: &mut crate::duration::Duration = &mut duration_0;
    let mut u8_3: u8 = crate::date::Date::sunday_based_week(date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5191() {
    rusty_monitor::set_test_id(5191);
    let mut i32_0: i32 = 49i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut u8_0: u8 = 6u8;
    let mut i32_1: i32 = -101i32;
    let mut u16_0: u16 = 37u16;
    let mut i32_2: i32 = -124i32;
    let mut i32_3: i32 = 62i32;
    let mut i64_0: i64 = 59i64;
    let mut u32_0: u32 = 89u32;
    let mut u8_1: u8 = 15u8;
    let mut u8_2: u8 = 99u8;
    let mut u8_3: u8 = 76u8;
    let mut i32_4: i32 = -27i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_4};
    let mut i32_5: i32 = 188i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_5};
    let mut i32_6: i32 = 54i32;
    let mut i64_1: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_6);
    let mut i64_2: i64 = 126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_7: i32 = 13i32;
    let mut i64_3: i64 = -52i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_7);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_1: u32 = 36u32;
    let mut u8_4: u8 = 59u8;
    let mut u8_5: u8 = 97u8;
    let mut u8_6: u8 = 70u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_6, u8_5, u8_4, u32_1);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_3: weekday::Weekday = std::clone::Clone::clone(weekday_1_ref_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    let mut i32_8: i32 = crate::duration::Duration::subsec_nanoseconds(duration_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_2);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_1, u8_3, u8_2, u8_1, u32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_3);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_2, u16_0);
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut result_3: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_0, weekday_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5356() {
    rusty_monitor::set_test_id(5356);
    let mut i8_0: i8 = 6i8;
    let mut i8_1: i8 = 9i8;
    let mut i8_2: i8 = -20i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 62i32;
    let mut i64_0: i64 = 59i64;
    let mut u32_0: u32 = 89u32;
    let mut u8_0: u8 = 15u8;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 76u8;
    let mut i32_1: i32 = -27i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 188i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = 54i32;
    let mut i64_1: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut i64_2: i64 = 126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_4: i32 = 13i32;
    let mut i64_3: i64 = -52i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_4);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_1: u32 = 36u32;
    let mut u8_3: u8 = 59u8;
    let mut u8_4: u8 = 97u8;
    let mut u8_5: u8 = 70u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_5, u8_4, u8_3, u32_1);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_2: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    let mut i32_5: i32 = crate::duration::Duration::subsec_nanoseconds(duration_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut tuple_1: (i8, i8, i8) = crate::utc_offset::UtcOffset::as_hms(utcoffset_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5951() {
    rusty_monitor::set_test_id(5951);
    let mut i8_0: i8 = -61i8;
    let mut i8_1: i8 = 58i8;
    let mut i8_2: i8 = -77i8;
    let mut u16_0: u16 = 37u16;
    let mut i32_0: i32 = -124i32;
    let mut i32_1: i32 = 62i32;
    let mut i64_0: i64 = 59i64;
    let mut u32_0: u32 = 89u32;
    let mut u8_0: u8 = 15u8;
    let mut u8_1: u8 = 99u8;
    let mut u8_2: u8 = 76u8;
    let mut i32_2: i32 = -27i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = 188i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut i32_4: i32 = 54i32;
    let mut i64_1: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_4);
    let mut i64_2: i64 = 126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_5: i32 = 13i32;
    let mut i64_3: i64 = -52i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_5);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_1: u32 = 36u32;
    let mut u8_3: u8 = 59u8;
    let mut u8_4: u8 = 97u8;
    let mut u8_5: u8 = 70u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_5, u8_4, u8_3, u32_1);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_2: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    let mut i32_6: i32 = crate::duration::Duration::subsec_nanoseconds(duration_0);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_0, u8_2, u8_1, u8_0, u32_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_ordinal_date(i32_0, u16_0);
    let mut padding_1: duration::Padding = crate::duration::Padding::Optimize;
    let mut result_3: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1271() {
    rusty_monitor::set_test_id(1271);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = -84.303905f64;
    let mut i32_0: i32 = -141i32;
    let mut i64_0: i64 = 66i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut f32_0: f32 = 80.838805f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i8_0: i8 = 15i8;
    let mut i8_1: i8 = -92i8;
    let mut i8_2: i8 = -15i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = -115i8;
    let mut i8_4: i8 = 17i8;
    let mut i8_5: i8 = 82i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = 38i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut i64_2: i64 = 71i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::weeks(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut i64_3: i64 = -69i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_4: i64 = 189i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_1: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_9_ref_0: &mut crate::duration::Duration = &mut duration_9;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_903() {
    rusty_monitor::set_test_id(903);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_0: u16 = 4u16;
    let mut i32_0: i32 = 1i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut i32_1: i32 = 188i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 54i32;
    let mut i64_0: i64 = -109i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_2);
    let mut i64_1: i64 = 126i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_3: i32 = 13i32;
    let mut i64_2: i64 = -52i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_3);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_0: u32 = 36u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 97u8;
    let mut u8_2: u8 = 70u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_nano(u8_2, u8_1, u8_0, u32_0);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_2: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_2);
    let mut i32_4: i32 = crate::duration::Duration::subsec_nanoseconds(duration_1);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4851() {
    rusty_monitor::set_test_id(4851);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut i32_0: i32 = 62i32;
    let mut i64_0: i64 = 59i64;
    let mut i32_1: i32 = 188i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 54i32;
    let mut i64_1: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut i64_2: i64 = 126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_3: i32 = 13i32;
    let mut i64_3: i64 = -52i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_2: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_3_ref_0: &mut crate::duration::Duration = &mut duration_3;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_1);
    let mut i32_4: i32 = crate::duration::Duration::subsec_nanoseconds(duration_0);
    let mut month_1: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_0);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_2: month::Month = crate::month::Month::June;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut month_3: month::Month = crate::month::Month::previous(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6338() {
    rusty_monitor::set_test_id(6338);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut i64_0: i64 = 31i64;
    let mut str_0: &str = "XGKtV09KiGcZe";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut u32_0: u32 = 0u32;
    let mut u8_0: u8 = 48u8;
    let mut u8_1: u8 = 55u8;
    let mut u8_2: u8 = 83u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = 28i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_0);
    let mut i32_0: i32 = 13i32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_3: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1874() {
    rusty_monitor::set_test_id(1874);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_0: i32 = 62i32;
    let mut i64_0: i64 = 59i64;
    let mut i32_1: i32 = -27i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 188i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = 54i32;
    let mut i64_1: i64 = -109i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut i64_2: i64 = 126i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i32_4: i32 = 13i32;
    let mut i64_3: i64 = -52i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_4);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_2: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_4_ref_0: &mut crate::duration::Duration = &mut duration_4;
    let mut i128_0: i128 = crate::duration::Duration::whole_microseconds(duration_2);
    let mut i32_5: i32 = crate::duration::Duration::subsec_nanoseconds(duration_1);
    let mut month_0: month::Month = crate::month::Month::October;
    let mut tuple_0: (i32, u8, weekday::Weekday) = crate::date::Date::to_iso_week_date(date_1);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut month_1: month::Month = crate::month::Month::June;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4162() {
    rusty_monitor::set_test_id(4162);
    let mut i64_0: i64 = -6i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i8_0: i8 = 58i8;
    let mut i8_1: i8 = 114i8;
    let mut i8_2: i8 = -75i8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut i32_0: i32 = 16i32;
    let mut i64_1: i64 = 114i64;
    let mut i32_1: i32 = -51i32;
    let mut f32_0: f32 = 51.538979f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut f64_0: f64 = -43.561003f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_2: i64 = 84i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i64_3: i64 = -109i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut i64_4: i64 = 126i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_2: i32 = 13i32;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_2);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut f64_1: f64 = 169.057512f64;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_0);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut weekday_4: weekday::Weekday = std::clone::Clone::clone(weekday_1_ref_0);
    let mut duration_8_ref_0: &mut crate::duration::Duration = &mut duration_8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_3: i32 = crate::duration::Duration::subsec_microseconds(duration_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1763() {
    rusty_monitor::set_test_id(1763);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut i32_0: i32 = 38i32;
    let mut i64_0: i64 = 45i64;
    let mut i8_0: i8 = 60i8;
    let mut i8_1: i8 = -65i8;
    let mut i8_2: i8 = -62i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i64_1: i64 = -109i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_0, i32_0);
    let mut i64_2: i64 = 126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i32_1: i32 = 13i32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
    let mut f64_0: f64 = 169.057512f64;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut weekday_5: weekday::Weekday = std::clone::Clone::clone(weekday_2_ref_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5804() {
    rusty_monitor::set_test_id(5804);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = -109i8;
    let mut i8_1: i8 = -41i8;
    let mut i8_2: i8 = 29i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 56i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut i64_0: i64 = -25i64;
    let mut i32_1: i32 = 45i32;
    let mut i64_1: i64 = 14i64;
    let mut i8_3: i8 = -21i8;
    let mut i8_4: i8 = 43i8;
    let mut i8_5: i8 = -2i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 27i8;
    let mut i8_7: i8 = -34i8;
    let mut i8_8: i8 = 67i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i32_2: i32 = 45i32;
    let mut i64_2: i64 = -153i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_2);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_1);
    let mut i64_3: i64 = 126i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_3: i32 = 13i32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_3);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_3: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut duration_2_ref_0: &mut crate::duration::Duration = &mut duration_2;
    let mut u8_0: u8 = crate::primitive_date_time::PrimitiveDateTime::monday_based_week(primitivedatetime_1);
    let mut i128_0: i128 = crate::offset_date_time::OffsetDateTime::unix_timestamp_nanos(offsetdatetime_0);
    panic!("From RustyUnit with love");
}
}