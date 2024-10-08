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
fn rusty_test_7446() {
    rusty_monitor::set_test_id(7446);
    let mut u16_0: u16 = 79u16;
    let mut u8_0: u8 = 23u8;
    let mut u8_1: u8 = 42u8;
    let mut u8_2: u8 = 4u8;
    let mut i8_0: i8 = -69i8;
    let mut i8_1: i8 = -25i8;
    let mut i8_2: i8 = -1i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i64_0: i64 = 81i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i32_0: i32 = -7i32;
    let mut i64_1: i64 = 27i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut i64_2: i64 = 81i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds(i64_2);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_2, duration_1);
    let mut i8_3: i8 = 56i8;
    let mut i8_4: i8 = 96i8;
    let mut i8_5: i8 = 32i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f64_0: f64 = -15.766206f64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut f64_1: f64 = 3.198145f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut f32_0: f32 = -132.125126f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_5);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut u32_0: u32 = 32u32;
    let mut u8_3: u8 = 80u8;
    let mut u8_4: u8 = 5u8;
    let mut u8_5: u8 = 52u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_0);
    let mut i32_1: i32 = 65i32;
    let mut i64_3: i64 = -10i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut i8_6: i8 = -54i8;
    let mut i8_7: i8 = -102i8;
    let mut i8_8: i8 = -74i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_10: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut f32_1: f32 = -60.910676f32;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_13: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_13, duration_9);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_2);
    let mut i32_2: i32 = 107i32;
    let mut i64_4: i64 = -77i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_2);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_16: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_16, duration_15);
    let mut i32_3: i32 = -119i32;
    let mut i64_5: i64 = 88i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_3);
    let mut duration_19: std::time::Duration = crate::duration::Duration::abs_std(duration_18);
    let mut i64_6: i64 = 7i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_3, duration_20);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_4);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i64_7: i64 = 128i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut i32_4: i32 = -91i32;
    let mut i64_8: i64 = -63i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_8, i32_4);
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_22, duration_21);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_6, duration_23);
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_7);
    let mut i8_9: i8 = 73i8;
    let mut i8_10: i8 = 78i8;
    let mut i8_11: i8 = -43i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_8, utcoffset_5);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_9);
    let mut instant_3: crate::instant::Instant = crate::instant::Instant::now();
    let mut instant_4: crate::instant::Instant = crate::instant::Instant::now();
    let mut u16_1: u16 = 2u16;
    let mut i32_5: i32 = -72i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut i64_9: i64 = 16i64;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::days(i64_9);
    let mut u32_1: u32 = 31u32;
    let mut u8_6: u8 = 58u8;
    let mut u8_7: u8 = 7u8;
    let mut u8_8: u8 = 88u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_1);
    let mut i32_6: i32 = 28i32;
    let mut i64_10: i64 = 99i64;
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::new(i64_10, i32_6);
    let mut i8_12: i8 = 50i8;
    let mut i8_13: i8 = 104i8;
    let mut i8_14: i8 = 88i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_11: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_10, utcoffset_6);
    let mut date_2: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_11);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_25);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_3, time_3);
    let mut bool_0: bool = crate::utc_offset::UtcOffset::is_positive(utcoffset_3);
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_14, duration_12);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_milli(date_0, u8_2, u8_1, u8_0, u16_0);
    panic!("From RustyUnit with love");
}
}