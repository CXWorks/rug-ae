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
fn rusty_test_4751() {
    rusty_monitor::set_test_id(4751);
    let mut i8_0: i8 = -108i8;
    let mut i8_1: i8 = 5i8;
    let mut i8_2: i8 = 61i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 19u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 69u8;
    let mut u8_2: u8 = 81u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_0: i32 = -52i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_0);
    let mut u32_1: u32 = 17u32;
    let mut u8_3: u8 = 59u8;
    let mut u8_4: u8 = 21u8;
    let mut u8_5: u8 = 11u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_0: i64 = 135i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut u16_0: u16 = 24u16;
    let mut i32_1: i32 = 53i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    panic!("From RustyUnit with love");
}
}