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
#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_8290() {
    rusty_monitor::set_test_id(8290);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut i64_0: i64 = 76i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut u32_0: u32 = 98u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 50u8;
    let mut u8_2: u8 = 38u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -52i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = 76i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_0: i32 = 95i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_4);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i64_3: i64 = 45i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i8_0: i8 = -3i8;
    let mut i8_1: i8 = 27i8;
    let mut i8_2: i8 = -65i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -55i8;
    let mut i8_4: i8 = -28i8;
    let mut i8_5: i8 = -39i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u16_0: u16 = 83u16;
    let mut i32_1: i32 = 150i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_5);
    let mut i32_2: i32 = 35i32;
    let mut i64_4: i64 = -205i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_5: i64 = 68i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u32_1: u32 = crate::offset_date_time::OffsetDateTime::nanosecond(offsetdatetime_1);
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(weekday_1_ref_0, weekday_0_ref_0);
    let mut padding_1: time::Padding = crate::time::Padding::Optimize;
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7398() {
    rusty_monitor::set_test_id(7398);
    let mut i64_0: i64 = 172i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i64_1: i64 = 86i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_1, duration_0);
    let mut i32_0: i32 = -21i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut weekday_0: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut i32_1: i32 = 224i32;
    let mut i64_2: i64 = 133i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_1);
    let mut i32_2: i32 = 98i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut date_2: crate::date::Date = crate::date::Date::saturating_add(date_1, duration_3);
    let mut i128_0: i128 = 19i128;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_3: i64 = -6i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i32_3: i32 = -158i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_4: crate::date::Date = crate::date::Date::saturating_add(date_3, duration_4);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut i128_1: i128 = -99i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i64_4: i64 = 13i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut i32_4: i32 = -42i32;
    let mut i64_5: i64 = 311i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_4);
    let mut u8_0: u8 = crate::time::Time::minute(time_0);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut bool_0: bool = std::cmp::PartialEq::eq(weekday_1_ref_0, weekday_0_ref_0);
    let mut i128_2: i128 = crate::duration::Duration::whole_microseconds(duration_2);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7285() {
    rusty_monitor::set_test_id(7285);
    let mut i64_0: i64 = 76i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut u32_0: u32 = 98u32;
    let mut u8_0: u8 = 24u8;
    let mut u8_1: u8 = 50u8;
    let mut u8_2: u8 = 38u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_1: i64 = -52i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_2: i64 = 76i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut i32_0: i32 = 95i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_4);
    let mut time_1: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i64_3: i64 = 45i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i8_0: i8 = -3i8;
    let mut i8_1: i8 = 27i8;
    let mut i8_2: i8 = -65i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -55i8;
    let mut i8_4: i8 = -28i8;
    let mut i8_5: i8 = -39i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u16_0: u16 = 83u16;
    let mut i32_1: i32 = 150i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_2, utcoffset_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_3, utcoffset_0);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_5);
    let mut i32_2: i32 = 35i32;
    let mut i64_4: i64 = -205i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_2);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::abs(duration_6);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_5: i64 = 68i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
    let mut u32_1: u32 = 46u32;
    let mut u8_3: u8 = 61u8;
    let mut u8_4: u8 = 92u8;
    let mut u8_5: u8 = 3u8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u32_2: u32 = crate::offset_date_time::OffsetDateTime::nanosecond(offsetdatetime_1);
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut weekday_1: weekday::Weekday = std::clone::Clone::clone(weekday_0_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_321() {
    rusty_monitor::set_test_id(321);
    let mut i32_0: i32 = -126i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::previous(weekday_0);
    let mut weekday_1_ref_0: &weekday::Weekday = &mut weekday_1;
    let mut f64_0: f64 = 38.013246f64;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_0: i64 = 162i64;
    let mut i64_1: i64 = -82i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_1, duration_0);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = 19i8;
    let mut i8_2: i8 = -27i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i8_3: i8 = -23i8;
    let mut i8_4: i8 = -55i8;
    let mut i8_5: i8 = 8i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_2: i64 = -31i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut u32_0: u32 = 76u32;
    let mut u8_0: u8 = 91u8;
    let mut u8_1: u8 = 93u8;
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_6: i8 = -3i8;
    let mut i8_7: i8 = -69i8;
    let mut i8_8: i8 = 32i8;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_8, i8_7);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u8_2: u8 = 81u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::previous(weekday_2);
    let mut weekday_4_ref_0: &weekday::Weekday = &mut weekday_4;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(weekday_1_ref_0);
    panic!("From RustyUnit with love");
}

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_1558() {
    rusty_monitor::set_test_id(1558);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_0_ref_0: &weekday::Weekday = &mut weekday_0;
    let mut f64_0: f64 = 85.862450f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut weekday_1: weekday::Weekday = crate::date::Date::weekday(date_0);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut weekday_2_ref_0: &weekday::Weekday = &mut weekday_2;
    let mut i8_0: i8 = 37i8;
    let mut i8_1: i8 = -37i8;
    let mut i8_2: i8 = -44i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = -164i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i8_3: i8 = -127i8;
    let mut i8_4: i8 = 110i8;
    let mut i8_5: i8 = -54i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_1: i64 = -114i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i8_6: i8 = -14i8;
    let mut i8_7: i8 = 64i8;
    let mut i8_8: i8 = 25i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 89u32;
    let mut u8_0: u8 = 36u8;
    let mut u8_1: u8 = 72u8;
    let mut u8_2: u8 = 51u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = -51i8;
    let mut i8_10: i8 = -86i8;
    let mut i8_11: i8 = 6i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_2: i64 = -96i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i8_12: i8 = -56i8;
    let mut i8_13: i8 = 43i8;
    let mut i8_14: i8 = -116i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_1: u32 = 3u32;
    let mut u8_3: u8 = 26u8;
    let mut u8_4: u8 = 34u8;
    let mut u8_5: u8 = 67u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_3: i64 = 0i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i32_0: i32 = -33i32;
    let mut i64_4: i64 = -66i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_5, duration_4);
    let mut i8_15: i8 = -66i8;
    let mut i8_16: i8 = 49i8;
    let mut i8_17: i8 = 37i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = -13i8;
    let mut i8_19: i8 = 53i8;
    let mut i8_20: i8 = 20i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u32_2: u32 = 85u32;
    let mut u8_6: u8 = 64u8;
    let mut u8_7: u8 = 63u8;
    let mut u8_8: u8 = 89u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::abs(duration_8);
    let mut i8_21: i8 = 84i8;
    let mut i8_22: i8 = -43i8;
    let mut i8_23: i8 = 3i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i32_1: i32 = 33i32;
    let mut i64_5: i64 = -203i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i8_24: i8 = -70i8;
    let mut i8_25: i8 = -77i8;
    let mut i8_26: i8 = -16i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_8);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_10);
    let mut i8_27: i8 = 43i8;
    let mut i8_28: i8 = 67i8;
    let mut i8_29: i8 = -113i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i32_2: i32 = 68i32;
    let mut i64_6: i64 = -102i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_2);
    let mut u32_3: u32 = 99u32;
    let mut u8_9: u8 = 2u8;
    let mut u8_10: u8 = 13u8;
    let mut u8_11: u8 = 21u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i8_30: i8 = 18i8;
    let mut i8_31: i8 = 60i8;
    let mut i8_32: i8 = -6i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = -22i8;
    let mut i8_34: i8 = -46i8;
    let mut i8_35: i8 = 103i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_12: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut i8_36: i8 = -3i8;
    let mut i8_37: i8 = -69i8;
    let mut i8_38: i8 = 32i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut i64_7: i64 = -33i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::minutes(i64_7);
    let mut i8_39: i8 = -41i8;
    let mut i8_40: i8 = 74i8;
    let mut i8_41: i8 = 1i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut f64_1: f64 = 29.999469f64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut u32_4: u32 = 54u32;
    let mut u8_12: u8 = 81u8;
    let mut u8_13: u8 = 10u8;
    let mut u8_14: u8 = 84u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::previous(weekday_3);
    let mut weekday_4_ref_0: &weekday::Weekday = &mut weekday_4;
    let mut tuple_0: () = std::cmp::Eq::assert_receiver_is_total_eq(weekday_4_ref_0);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_2);
    let mut month_0: month::Month = crate::month::Month::April;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_1);
    let mut bool_0: bool = std::cmp::PartialEq::eq(weekday_2_ref_0, weekday_0_ref_0);
    panic!("From RustyUnit with love");
}
}