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
	use std::cmp::Eq;
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_645() {
    rusty_monitor::set_test_id(645);
    let mut i8_0: i8 = -116i8;
    let mut i8_1: i8 = -54i8;
    let mut i8_2: i8 = 36i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 133i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i32_1: i32 = 32i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 36i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_0: i64 = 71i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_0);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_2);
    let mut i64_1: i64 = -143i64;
    let mut i8_3: i8 = -40i8;
    let mut i8_4: i8 = -119i8;
    let mut i8_5: i8 = 56i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut bool_0: bool = true;
    let mut i64_2: i64 = -74i64;
    let mut i64_3: i64 = -48i64;
    let mut i64_4: i64 = -36i64;
    let mut str_0: &str = "3b";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u8_0: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_3);
    let mut i8_6: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_1);
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(weekday_2_ref_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_1);
    let mut u8_1: u8 = crate::offset_date_time::OffsetDateTime::iso_week(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_920() {
    rusty_monitor::set_test_id(920);
    let mut i32_0: i32 = 32i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i32_1: i32 = 36i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i64_0: i64 = 71i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut i64_1: i64 = -143i64;
    let mut i8_0: i8 = -40i8;
    let mut i8_1: i8 = -119i8;
    let mut i8_2: i8 = 56i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut bool_0: bool = true;
    let mut i64_2: i64 = -81i64;
    let mut i64_3: i64 = -48i64;
    let mut i64_4: i64 = -36i64;
    let mut str_0: &str = "3b";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u8_0: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_3);
    let mut i8_3: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_0);
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(weekday_2_ref_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_280() {
    rusty_monitor::set_test_id(280);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut u16_0: u16 = 54u16;
    let mut i32_0: i32 = -119i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i32_1: i32 = 32i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i32_2: i32 = 36i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut i64_0: i64 = 71i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_0);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_1, date_2);
    let mut i64_1: i64 = -143i64;
    let mut i8_0: i8 = -40i8;
    let mut i8_1: i8 = -119i8;
    let mut i8_2: i8 = 56i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut bool_0: bool = true;
    let mut i64_2: i64 = -74i64;
    let mut i64_3: i64 = -48i64;
    let mut i64_4: i64 = -36i64;
    let mut str_0: &str = "3b";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_4, maximum: i64_3, value: i64_2, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::next(weekday_2);
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut u8_0: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_4);
    let mut i8_3: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_0);
    let mut weekday_3_ref_0: &weekday::Weekday = &mut weekday_3;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(weekday_3_ref_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_1);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_0);
    let mut weekday_5: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1123() {
    rusty_monitor::set_test_id(1123);
    let mut i8_0: i8 = 19i8;
    let mut i8_1: i8 = -71i8;
    let mut i8_2: i8 = 9i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = 46i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u32_0: u32 = 46u32;
    let mut u8_0: u8 = 56u8;
    let mut u8_1: u8 = 73u8;
    let mut u8_2: u8 = 5u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -121i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut i32_0: i32 = -250i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_0};
    let mut i32_1: i32 = 36i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i64_2: i64 = 71i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_2);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_2);
    let mut i64_3: i64 = -143i64;
    let mut i8_3: i8 = -40i8;
    let mut i8_4: i8 = -119i8;
    let mut i8_5: i8 = 56i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut bool_0: bool = true;
    let mut i64_4: i64 = -74i64;
    let mut i64_5: i64 = -48i64;
    let mut i64_6: i64 = -36i64;
    let mut str_0: &str = "3b";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_6, maximum: i64_5, value: i64_4, conditional_range: bool_0};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u8_3: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_3);
    let mut i8_6: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_1);
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(weekday_2_ref_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i64_7: i64 = crate::duration::Duration::whole_seconds(duration_3);
    let mut u16_0: u16 = crate::offset_date_time::OffsetDateTime::millisecond(offsetdatetime_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1355() {
    rusty_monitor::set_test_id(1355);
    let mut i64_0: i64 = -146i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i32_0: i32 = 32i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i32_1: i32 = 36i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i64_1: i64 = 71i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_1);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut i64_2: i64 = -143i64;
    let mut i8_0: i8 = -40i8;
    let mut i8_1: i8 = -119i8;
    let mut i8_2: i8 = 56i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut str_0: &str = "3b";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u8_0: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_3);
    let mut i8_3: i8 = crate::utc_offset::UtcOffset::minutes_past_hour(utcoffset_0);
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(weekday_2_ref_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut f64_0: f64 = crate::duration::Duration::as_seconds_f64(duration_0);
    panic!("From RustyUnit with love");
}
}