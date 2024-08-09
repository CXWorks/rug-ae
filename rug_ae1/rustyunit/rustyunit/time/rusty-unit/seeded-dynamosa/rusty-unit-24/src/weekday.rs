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
	use std::cmp::PartialEq;
	use std::clone::Clone;
	use std::cmp::Eq;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1787() {
//    rusty_monitor::set_test_id(1787);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::next(weekday_2);
    let mut weekday_3_ref_0: &weekday::Weekday = &mut weekday_3;
    let mut i32_0: i32 = 1721119i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut weekday_4: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_0);
    let mut weekday_4_ref_0: &weekday::Weekday = &mut weekday_4;
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_6: weekday::Weekday = crate::weekday::Weekday::previous(weekday_5);
    let mut weekday_6_ref_0: &weekday::Weekday = &mut weekday_6;
    let mut weekday_7: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_7_ref_0: &weekday::Weekday = &mut weekday_7;
    let mut f32_0: f32 = 181.797308f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut weekday_8: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut weekday_8_ref_0: &weekday::Weekday = &mut weekday_8;
    let mut weekday_9: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_9_ref_0: &weekday::Weekday = &mut weekday_9;
    let mut weekday_10: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_10_ref_0: &weekday::Weekday = &mut weekday_10;
    let mut weekday_11: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_12: weekday::Weekday = crate::weekday::Weekday::previous(weekday_11);
    let mut weekday_13: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_13_ref_0: &weekday::Weekday = &mut weekday_13;
    let mut bool_0: bool = std::cmp::PartialEq::eq(weekday_10_ref_0, weekday_9_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(weekday_8_ref_0, weekday_7_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(weekday_6_ref_0, weekday_4_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(weekday_3_ref_0, weekday_1_ref_0);
    let mut weekday_14: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1380() {
//    rusty_monitor::set_test_id(1380);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::previous(weekday_3);
    let mut weekday_4_ref_0: &weekday::Weekday = &mut weekday_4;
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_5_ref_0: &weekday::Weekday = &mut weekday_5;
    let mut f32_0: f32 = 181.797308f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut weekday_6: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut weekday_6_ref_0: &weekday::Weekday = &mut weekday_6;
    let mut weekday_7: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_7_ref_0: &weekday::Weekday = &mut weekday_7;
    let mut weekday_8: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_8_ref_0: &weekday::Weekday = &mut weekday_8;
    let mut weekday_9: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_9_ref_0: &weekday::Weekday = &mut weekday_9;
    let mut bool_0: bool = std::cmp::PartialEq::eq(weekday_8_ref_0, weekday_7_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(weekday_6_ref_0, weekday_5_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(weekday_2_ref_0, weekday_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_556() {
//    rusty_monitor::set_test_id(556);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut u16_0: u16 = 59u16;
    let mut i32_0: i32 = 314i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut weekday_1: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 0i8;
    let mut i8_2: i8 = -96i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 218i32;
    let mut i64_0: i64 = 42i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_1);
    let mut i128_0: i128 = 0i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_1, duration_0);
    let mut i64_1: i64 = 1i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_1);
    let mut i8_3: i8 = -104i8;
    let mut i8_4: i8 = 5i8;
    let mut i8_5: i8 = 24i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_2: i64 = 12i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(weekday_2_ref_0);
    let mut tuple_1: () = std::cmp::Eq::assert_receiver_is_total_eq(weekday_1_ref_0);
    let mut tuple_2: () = std::cmp::Eq::assert_receiver_is_total_eq(weekday_0_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_1519() {
//    rusty_monitor::set_test_id(1519);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u32_0: u32 = 1000u32;
    let mut u8_0: u8 = 9u8;
    let mut u8_1: u8 = 24u8;
    let mut u8_2: u8 = 9u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_0: i64 = 253402300799i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_0);
    let mut u16_0: u16 = 29u16;
    let mut i32_0: i32 = 116i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_0);
    let mut i128_0: i128 = 1i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut f64_0: f64 = 35.854337f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::previous(weekday_2);
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_4_ref_0: &weekday::Weekday = &mut weekday_4;
    let mut bool_0: bool = std::cmp::PartialEq::eq(weekday_0_ref_0, weekday_1_ref_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4962() {
//    rusty_monitor::set_test_id(4962);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_0: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
    let mut i32_0: i32 = 1721119i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut weekday_3: weekday::Weekday = crate::primitive_date_time::PrimitiveDateTime::weekday(primitivedatetime_0);
    let mut weekday_3_ref_0: &weekday::Weekday = &mut weekday_3;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::previous(weekday_4);
    let mut weekday_5_ref_0: &weekday::Weekday = &mut weekday_5;
    let mut f32_0: f32 = 181.797308f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_1);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut weekday_6: weekday::Weekday = crate::date::Date::weekday(date_1);
    let mut weekday_7: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_7_ref_0: &weekday::Weekday = &mut weekday_7;
    let mut weekday_8: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_8_ref_0: &weekday::Weekday = &mut weekday_8;
    let mut weekday_9: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_10: weekday::Weekday = crate::weekday::Weekday::previous(weekday_9);
    let mut weekday_10_ref_0: &weekday::Weekday = &mut weekday_10;
    let mut weekday_11: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_11_ref_0: &weekday::Weekday = &mut weekday_11;
    let mut bool_0: bool = std::cmp::PartialEq::eq(weekday_11_ref_0, weekday_10_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(weekday_8_ref_0, weekday_7_ref_0);
    let mut bool_2: bool = std::cmp::PartialEq::eq(weekday_5_ref_0, weekday_3_ref_0);
    let mut bool_3: bool = std::cmp::PartialEq::eq(weekday_2_ref_0, weekday_0_ref_0);
    let mut weekday_12: weekday::Weekday = crate::weekday::Weekday::next(weekday_6);
    let mut bool_4: bool = crate::duration::Duration::is_positive(duration_0);
    let mut u8_0: u8 = crate::offset_date_time::OffsetDateTime::monday_based_week(offsetdatetime_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4997() {
//    rusty_monitor::set_test_id(4997);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut u16_0: u16 = 96u16;
    let mut i32_0: i32 = 128i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut weekday_1: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut i64_0: i64 = 60i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_1: i32 = 212i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut i32_2: i32 = 336i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i64_1: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut duration_2_ref_0: &std::time::Duration = &mut duration_2;
    let mut i64_2: i64 = 16i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i64_3: i64 = 1000i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_4, duration_3);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_3_ref_0: &weekday::Weekday = &mut weekday_3;
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_6: weekday::Weekday = crate::weekday::Weekday::previous(weekday_4);
    let mut weekday_5_ref_0: &weekday::Weekday = &mut weekday_5;
    let mut weekday_7: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_6_ref_0: &weekday::Weekday = &mut weekday_6;
    let mut bool_0: bool = std::cmp::PartialEq::eq(weekday_2_ref_0, weekday_3_ref_0);
    let mut bool_1: bool = std::cmp::PartialEq::eq(weekday_5_ref_0, weekday_6_ref_0);
//    panic!("From RustyUnit with love");
}
}