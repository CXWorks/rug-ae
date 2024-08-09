//! Implementation of [`Distribution`] for various structs.

use rand::distributions::{Distribution, Standard};
use rand::Rng;

use crate::{Date, Duration, Month, OffsetDateTime, PrimitiveDateTime, Time, UtcOffset, Weekday};

impl Distribution<Time> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Time {
        Time::__from_hms_nanos_unchecked(
            rng.gen_range(0..24),
            rng.gen_range(0..60),
            rng.gen_range(0..60),
            rng.gen_range(0..1_000_000_000),
        )
    }
}

impl Distribution<Date> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Date {
        Date::from_julian_day_unchecked(
            rng.gen_range(Date::MIN.to_julian_day()..=Date::MAX.to_julian_day()),
        )
    }
}

impl Distribution<UtcOffset> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> UtcOffset {
        let seconds = rng.gen_range(-86399..=86399);
        UtcOffset::__from_hms_unchecked(
            (seconds / 3600) as _,
            ((seconds % 3600) / 60) as _,
            (seconds % 60) as _,
        )
    }
}

impl Distribution<PrimitiveDateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PrimitiveDateTime {
        PrimitiveDateTime::new(Self.sample(rng), Self.sample(rng))
    }
}

impl Distribution<OffsetDateTime> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> OffsetDateTime {
        let date_time: PrimitiveDateTime = Self.sample(rng);
        date_time.assume_offset(Self.sample(rng))
    }
}

impl Distribution<Duration> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Duration {
        Duration::nanoseconds_i128(
            rng.gen_range(Duration::MIN.whole_nanoseconds()..=Duration::MAX.whole_nanoseconds()),
        )
    }
}

impl Distribution<Weekday> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Weekday {
        use Weekday::*;

        match rng.gen_range(0..7) {
            0 => Monday,
            1 => Tuesday,
            2 => Wednesday,
            3 => Thursday,
            4 => Friday,
            5 => Saturday,
            _ => Sunday,
        }
    }
}

impl Distribution<Month> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Month {
        use Month::*;
        match rng.gen_range(1..=12) {
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
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8966() {
//    rusty_monitor::set_test_id(8966);
    let mut i64_0: i64 = 60i64;
    let mut i64_1: i64 = 86400i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_0: i8 = -24i8;
    let mut i8_1: i8 = 2i8;
    let mut i8_2: i8 = 0i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 392i32;
    let mut i64_2: i64 = -84i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut u32_0: u32 = 100000000u32;
    let mut u8_0: u8 = 12u8;
    let mut u8_1: u8 = 8u8;
    let mut u8_2: u8 = 6u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i64_3: i64 = 12i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut f64_0: f64 = 64.323926f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut f64_1: f64 = 4768169126130614272.000000f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_4: i64 = 2440588i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_7: std::time::Duration = crate::duration::Duration::abs_std(duration_6);
    let mut i8_3: i8 = 24i8;
    let mut i8_4: i8 = 60i8;
    let mut i8_5: i8 = 23i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i8_6: i8 = 2i8;
    let mut i8_7: i8 = 23i8;
    let mut i8_8: i8 = 0i8;
    let mut i8_9: i8 = 24i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_9, i8_8, i8_7);
    let mut i64_5: i64 = 1000000000i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i32_1: i32 = 161i32;
    let mut i64_6: i64 = 60i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new(i64_6, i32_1);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i8_10: i8 = 0i8;
    let mut i8_11: i8 = 6i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_6, i8_11, i8_10);
    let mut i64_7: i64 = 59i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut duration_14: std::time::Duration = crate::duration::Duration::abs_std(duration_13);
    let mut i128_0: i128 = 36i128;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_8: i64 = 253402300799i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::minutes(i64_8);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::abs(duration_16);
    let mut duration_18: std::time::Duration = crate::duration::Duration::abs_std(duration_17);
    let mut f64_2: f64 = 4794699203894837248.000000f64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut i64_9: i64 = 1000000000i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::seconds(i64_9);
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_20, duration_19);
    let mut duration_22: std::time::Duration = crate::duration::Duration::abs_std(duration_21);
    let mut u32_1: u32 = 100u32;
    let mut u8_3: u8 = 1u8;
    let mut u8_4: u8 = 12u8;
    let mut u8_5: u8 = 39u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut u32_2: u32 = 49u32;
    let mut u8_6: u8 = 0u8;
    let mut u8_7: u8 = 60u8;
    let mut u8_8: u8 = 53u8;
    let mut i32_2: i32 = 82i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut u32_3: u32 = 10000000u32;
    let mut u8_9: u8 = 8u8;
    let mut u8_10: u8 = 8u8;
    let mut u8_11: u8 = 60u8;
    let mut i32_3: i32 = 161i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_3};
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_1, u8_11, u8_10, u8_9, u32_3);
    let mut result_1: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_micro(date_0, u8_8, u8_7, u8_6, u32_2);
//    panic!("From RustyUnit with love");
}
}