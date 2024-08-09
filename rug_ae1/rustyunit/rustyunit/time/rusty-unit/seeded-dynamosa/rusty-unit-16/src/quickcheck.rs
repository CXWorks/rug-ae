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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_4755() {
//    rusty_monitor::set_test_id(4755);
    let mut i32_0: i32 = -66i32;
    let mut i64_0: i64 = 1i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_0, i32_0);
    let mut i8_0: i8 = 113i8;
    let mut i8_1: i8 = 60i8;
    let mut i8_2: i8 = 60i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_3: i8 = 127i8;
    let mut i8_4: i8 = -85i8;
    let mut i8_5: i8 = 2i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 4i8;
    let mut i8_7: i8 = 56i8;
    let mut i8_8: i8 = 59i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 999999u32;
    let mut u8_0: u8 = 2u8;
    let mut u8_1: u8 = 71u8;
    let mut u8_2: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = 115i8;
    let mut i8_10: i8 = 1i8;
    let mut i8_11: i8 = 24i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_1: i64 = 24i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut i64_2: i64 = 24i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut i64_3: i64 = 1000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_3, duration_2);
    let mut i8_12: i8 = 24i8;
    let mut i8_13: i8 = 2i8;
    let mut i8_14: i8 = -47i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_4: i64 = 60i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut i8_15: i8 = 60i8;
    let mut i8_16: i8 = 1i8;
    let mut i8_17: i8 = 0i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i64_5: i64 = 12i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_7, duration_6);
    let mut u32_1: u32 = 1000000u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 9u8;
    let mut u8_5: u8 = 31u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_6: i64 = -101i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_10, duration_9);
    let mut i8_18: i8 = 127i8;
    let mut i8_19: i8 = 1i8;
    let mut i8_20: i8 = 0i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut u32_2: u32 = 999999999u32;
    let mut u8_6: u8 = 52u8;
    let mut u8_7: u8 = 59u8;
    let mut u8_8: u8 = 11u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i128_1: i128 = 46i128;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i8_21: i8 = 1i8;
    let mut i8_22: i8 = 43i8;
    let mut i8_23: i8 = 24i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_3: u32 = 1000000000u32;
    let mut u8_9: u8 = 7u8;
    let mut u8_10: u8 = 62u8;
    let mut u8_11: u8 = 6u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i8_24: i8 = 4i8;
    let mut i8_25: i8 = 0i8;
    let mut i8_26: i8 = 0i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i8_27: i8 = 24i8;
    let mut i8_28: i8 = 24i8;
    let mut i8_29: i8 = 19i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut f32_0: f32 = 1315859240.000000f32;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_7: i64 = 9223372036854775807i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::microseconds(i64_7);
    let mut i8_30: i8 = 6i8;
    let mut i8_31: i8 = 3i8;
    let mut i8_32: i8 = 3i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = 1i8;
    let mut i8_34: i8 = 1i8;
    let mut i8_35: i8 = 5i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i32_1: i32 = -54i32;
    let mut i64_8: i64 = 1000000000i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::days(i64_8);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_15, i32_1);
    let mut i64_9: i64 = -159i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_9);
    let mut i8_36: i8 = 59i8;
    let mut i8_37: i8 = -22i8;
    let mut i8_38: i8 = 2i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut u32_4: u32 = 100000000u32;
    let mut u8_12: u8 = 30u8;
    let mut u8_13: u8 = 6u8;
    let mut u8_14: u8 = 0u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i8_39: i8 = 60i8;
    let mut i8_40: i8 = 6i8;
    let mut i8_41: i8 = 1i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut u32_5: u32 = 100000000u32;
    let mut u8_15: u8 = 11u8;
    let mut u8_16: u8 = 7u8;
    let mut u8_17: u8 = 28u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_18: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i8_42: i8 = 60i8;
    let mut i8_43: i8 = 1i8;
    let mut i8_44: i8 = 23i8;
    let mut utcoffset_14: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut i32_2: i32 = 263i32;
    let mut i64_10: i64 = 12i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_10, i32_2);
    let mut i8_45: i8 = 1i8;
    let mut i8_46: i8 = 59i8;
    let mut i8_47: i8 = 1i8;
    let mut utcoffset_15: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_47, i8_46, i8_45);
    let mut i32_3: i32 = 305i32;
    let mut i64_11: i64 = 1000000i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::new(i64_11, i32_3);
    let mut i64_12: i64 = 1000000i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::hours(i64_12);
    let mut u32_6: u32 = 10000000u32;
    let mut u8_18: u8 = 98u8;
    let mut u8_19: u8 = 4u8;
    let mut u8_20: u8 = 44u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut i8_48: i8 = 60i8;
    let mut i8_49: i8 = 1i8;
    let mut i8_50: i8 = 127i8;
    let mut utcoffset_16: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_50, i8_49, i8_48);
    let mut i8_51: i8 = 5i8;
    let mut i8_52: i8 = 2i8;
    let mut i8_53: i8 = 1i8;
    let mut utcoffset_17: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_53, i8_52, i8_51);
    let mut i64_13: i64 = 1000i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::minutes(i64_13);
    let mut i64_14: i64 = 86400i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::days(i64_14);
    let mut duration_24: std::time::Duration = crate::duration::Duration::abs_std(duration_23);
    let mut i64_15: i64 = 1000000000i64;
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::weeks(i64_15);
    let mut duration_26: std::time::Duration = crate::duration::Duration::abs_std(duration_25);
    let mut i64_16: i64 = 24i64;
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::seconds(i64_16);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut u8_21: u8 = 24u8;
    let mut i32_4: i32 = 36525i32;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::previous(weekday_1);
    let mut u8_22: u8 = 5u8;
    let mut i32_5: i32 = -46i32;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut u8_23: u8 = 78u8;
    let mut i32_6: i32 = 25i32;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u8_24: u8 = 12u8;
    let mut i32_7: i32 = 342i32;
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut u8_25: u8 = 12u8;
    let mut i32_8: i32 = 314i32;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_8, u8_25, weekday_5);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_7, u8_24, weekday_4);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_6, u8_23, weekday_3);
    let mut result_3: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_5, u8_22, weekday_2);
    let mut result_4: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_4, u8_21, weekday_0);
//    panic!("From RustyUnit with love");
}
}