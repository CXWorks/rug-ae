//! Implementations of the [`quickcheck::Arbitrary`](quickcheck_dep::Arbitrary) trait.
//!
//! This enables users to write tests such as this, and have test values provided automatically:
//!
//! ```
//! # #![allow(dead_code)]
//! # use quickcheck_dep::quickcheck;
//! # #[cfg(pretend_we_didnt_rename_the_dependency)]
//! use quickcheck::quickcheck;
//! use time::Date;
//!
//! struct DateRange {
//!     from: Date,
//!     to: Date,
//! }
//!
//! impl DateRange {
//!     fn new(from: Date, to: Date) -> Result<Self, ()> {
//!         Ok(DateRange { from, to })
//!     }
//! }
//!
//! quickcheck! {
//!     fn date_range_is_well_defined(from: Date, to: Date) -> bool {
//!         let r = DateRange::new(from, to);
//!         if from <= to {
//!             r.is_ok()
//!         } else {
//!             r.is_err()
//!         }
//!     }
//! }
//! ```
//!
//! An implementation for `Instant` is intentionally omitted since its values are only meaningful in
//! relation to a [`Duration`], and obtaining an `Instant` from a [`Duration`] is very simple
//! anyway.

use alloc::boxed::Box;

use quickcheck_dep::{empty_shrinker, single_shrinker, Arbitrary, Gen};

use crate::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

/// Obtain an arbitrary value between the minimum and maximum inclusive.
macro_rules! arbitrary_between {
    ($type:ty; $gen:expr, $min:expr, $max:expr) => {{
        let min = $min;
        let max = $max;
        let range = max - min;
        <$type>::arbitrary($gen).rem_euclid(range + 1) + min
    }};
}

impl Arbitrary for Date {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::from_julian_day_unchecked(arbitrary_between!(
            i32;
            g,
            Self::MIN.to_julian_day(),
            Self::MAX.to_julian_day()
        ))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.to_ordinal_date()
                .shrink()
                .flat_map(|(year, ordinal)| Self::from_ordinal_date(year, ordinal)),
        )
    }
}

impl Arbitrary for Duration {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::nanoseconds_i128(arbitrary_between!(
            i128;
            g,
            Self::MIN.whole_nanoseconds(),
            Self::MAX.whole_nanoseconds()
        ))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.subsec_nanoseconds(), self.whole_seconds())
                .shrink()
                .map(|(mut nanoseconds, seconds)| {
                    // Coerce the sign if necessary.
                    if (seconds > 0 && nanoseconds < 0) || (seconds < 0 && nanoseconds > 0) {
                        nanoseconds *= -1;
                    }

                    Self::new_unchecked(seconds, nanoseconds)
                }),
        )
    }
}

impl Arbitrary for Time {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::__from_hms_nanos_unchecked(
            arbitrary_between!(u8; g, 0, 23),
            arbitrary_between!(u8; g, 0, 59),
            arbitrary_between!(u8; g, 0, 59),
            arbitrary_between!(u32; g, 0, 999_999_999),
        )
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.as_hms_nano()
                .shrink()
                .map(|(hour, minute, second, nanosecond)| {
                    Self::__from_hms_nanos_unchecked(hour, minute, second, nanosecond)
                }),
        )
    }
}

impl Arbitrary for PrimitiveDateTime {
    fn arbitrary(g: &mut Gen) -> Self {
        Self::new(<_>::arbitrary(g), <_>::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.date, self.time)
                .shrink()
                .map(|(date, time)| Self { date, time }),
        )
    }
}

impl Arbitrary for UtcOffset {
    fn arbitrary(g: &mut Gen) -> Self {
        let seconds = arbitrary_between!(i32; g, -86_399, 86_399);
        Self::__from_hms_unchecked(
            (seconds / 3600) as _,
            ((seconds % 3600) / 60) as _,
            (seconds % 60) as _,
        )
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            self.as_hms().shrink().map(|(hours, minutes, seconds)| {
                Self::__from_hms_unchecked(hours, minutes, seconds)
            }),
        )
    }
}

impl Arbitrary for OffsetDateTime {
    fn arbitrary(g: &mut Gen) -> Self {
        let datetime = PrimitiveDateTime::arbitrary(g);
        datetime.assume_offset(<_>::arbitrary(g))
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        Box::new(
            (self.utc_datetime.utc_to_offset(self.offset), self.offset)
                .shrink()
                .map(|(utc_datetime, offset)| utc_datetime.assume_offset(offset)),
        )
    }
}

impl Arbitrary for Weekday {
    fn arbitrary(g: &mut Gen) -> Self {
        use Weekday::*;
        match arbitrary_between!(u8; g, 0, 6) {
            0 => Monday,
            1 => Tuesday,
            2 => Wednesday,
            3 => Thursday,
            4 => Friday,
            5 => Saturday,
            _ => Sunday,
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            Self::Monday => empty_shrinker(),
            _ => single_shrinker(self.previous()),
        }
    }
}

impl Arbitrary for Month {
    fn arbitrary(g: &mut Gen) -> Self {
        use Month::*;
        match arbitrary_between!(u8; g, 1, 12) {
            1 => January,
            2 => February,
            3 => March,
            4 => April,
            5 => May,
            6 => June,
            7 => July,
            8 => August,
            9 => September,
            10 => October,
            11 => November,
            _ => December,
        }
    }

    fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
        match self {
            Self::January => empty_shrinker(),
            _ => single_shrinker(self.previous()),
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_7635() {
    rusty_monitor::set_test_id(7635);
    let mut i32_0: i32 = -94i32;
    let mut i64_0: i64 = -40i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 1i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut i32_1: i32 = -118i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut i64_2: i64 = -133i64;
    let mut i64_3: i64 = 62i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut u32_0: u32 = 15u32;
    let mut u8_0: u8 = 68u8;
    let mut u8_1: u8 = 35u8;
    let mut u8_2: u8 = 71u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut u32_1: u32 = 23u32;
    let mut u8_3: u8 = 10u8;
    let mut u8_4: u8 = 78u8;
    let mut u8_5: u8 = 25u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_1);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_3);
    let mut u16_0: u16 = 74u16;
    let mut i32_2: i32 = 16i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_1, utcoffset_0);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u16_1: u16 = 26u16;
    let mut i32_3: i32 = 106i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_1);
    let mut u32_2: u32 = 64u32;
    let mut u8_6: u8 = 74u8;
    let mut u8_7: u8 = 82u8;
    let mut u8_8: u8 = 0u8;
    let mut i32_4: i32 = 114i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_4);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u8_9: u8 = 0u8;
    let mut i32_5: i32 = -133i32;
    let mut i64_4: i64 = 60i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i64_5: i64 = 215i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut i8_0: i8 = 118i8;
    let mut i8_1: i8 = -29i8;
    let mut i8_2: i8 = -31i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_3: u32 = 8u32;
    let mut u8_10: u8 = 30u8;
    let mut u8_11: u8 = 0u8;
    let mut u8_12: u8 = 86u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_12, u8_11, u8_10, u32_3);
    let mut i32_6: i32 = -80i32;
    let mut date_5: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_5, time_3);
    let mut u16_2: u16 = 63u16;
    let mut i32_7: i32 = -192i32;
    let mut date_6: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_2);
    let mut i32_8: i32 = 91i32;
    let mut i64_6: i64 = -94i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_8);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u32_4: u32 = 18u32;
    let mut u8_13: u8 = 71u8;
    let mut u8_14: u8 = 91u8;
    let mut u8_15: u8 = 64u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_15, u8_14, u8_13, u32_4);
    let mut i32_9: i32 = 32i32;
    let mut date_7: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_9);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_7, time_4);
    let mut month_0: month::Month = crate::date::Date::month(date_1);
    let mut tuple_0: (u8, u8, u8, u32) = crate::time::Time::as_hms_micro(time_2);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_5, u8_9, weekday_1);
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_4, u8_8, u8_7, u8_6, u32_2);
    let mut time_0_ref_0: &crate::time::Time = &mut time_0;
    panic!("From RustyUnit with love");
}
}