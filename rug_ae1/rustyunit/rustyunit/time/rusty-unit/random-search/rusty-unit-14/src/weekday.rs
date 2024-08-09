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
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_2965() {
    rusty_monitor::set_test_id(2965);
    let mut i8_0: i8 = 27i8;
    let mut i8_1: i8 = 12i8;
    let mut i8_2: i8 = 126i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_0: f64 = 144.993619f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_0: i32 = 14i32;
    let mut i64_0: i64 = -64i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_1: i32 = -1i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_2);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i8_3: i8 = -2i8;
    let mut i8_4: i8 = -42i8;
    let mut i8_5: i8 = 15i8;
    let mut i8_6: i8 = -80i8;
    let mut i8_7: i8 = -17i8;
    let mut i8_8: i8 = 103i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i32_2: i32 = -94i32;
    let mut i64_1: i64 = 156i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_2);
    let mut u32_0: u32 = 66u32;
    let mut u8_0: u8 = 46u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 21u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_3: i32 = -100i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_1, duration_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(weekday_0_ref_0);
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::millisecond(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_5, i8_4, i8_3);
    let mut weekday_1: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_0);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::None;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1884() {
    rusty_monitor::set_test_id(1884);
    let mut f64_0: f64 = -109.115422f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut month_0: month::Month = crate::offset_date_time::OffsetDateTime::month(offsetdatetime_1);
    let mut u32_0: u32 = 84u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 3u8;
    let mut u8_2: u8 = 49u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(weekday_0_ref_0);
    let mut u8_3: u8 = crate::time::Time::minute(time_0);
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_4035() {
    rusty_monitor::set_test_id(4035);
    let mut i8_0: i8 = 26i8;
    let mut i8_1: i8 = 19i8;
    let mut i8_2: i8 = -67i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 67u8;
    let mut u8_1: u8 = 45u8;
    let mut u8_2: u8 = 30u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = -85i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut u16_0: u16 = 18u16;
    let mut i32_0: i32 = -121i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::utc_to_offset(primitivedatetime_0, utcoffset_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_0, primitivedatetime_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut weekday_1: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    let mut u8_3: u8 = crate::offset_date_time::OffsetDateTime::sunday_based_week(offsetdatetime_1);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3073() {
    rusty_monitor::set_test_id(3073);
    let mut i32_0: i32 = 96i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i32_1: i32 = 21i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut i8_0: i8 = 95i8;
    let mut i8_1: i8 = -68i8;
    let mut i8_2: i8 = -27i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u8_0: u8 = 17u8;
    let mut u8_1: u8 = 7u8;
    let mut u8_2: u8 = 20u8;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_2, u8_1, u8_0);
    let mut i8_3: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_0);
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
    let mut bool_0: bool = std::cmp::PartialEq::eq(weekday_2_ref_0, weekday_1_ref_0);
    let mut u8_3: u8 = crate::date::Date::iso_week(date_1);
    let mut weekday_3: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_3217() {
    rusty_monitor::set_test_id(3217);
    let mut i32_0: i32 = 110i32;
    let mut i64_0: i64 = -91i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i64_1: i64 = 61i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i8_0: i8 = 20i8;
    let mut i8_1: i8 = 28i8;
    let mut i8_2: i8 = -19i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 32i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_0, utcoffset_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(weekday_1_ref_0, weekday_0_ref_0);
    let mut option_0: std::option::Option<crate::offset_date_time::OffsetDateTime> = crate::offset_date_time::OffsetDateTime::checked_add(offsetdatetime_0, duration_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = std::option::Option::unwrap(option_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_0);
    panic!("From RustyUnit with love");
}
}