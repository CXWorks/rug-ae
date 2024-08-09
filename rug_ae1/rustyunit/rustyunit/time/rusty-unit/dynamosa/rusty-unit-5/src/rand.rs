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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_6810() {
    rusty_monitor::set_test_id(6810);
    let mut i64_0: i64 = -111i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i8_0: i8 = -127i8;
    let mut i8_1: i8 = 70i8;
    let mut i8_2: i8 = -89i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut i8_3: i8 = -83i8;
    let mut i8_4: i8 = 18i8;
    let mut i8_5: i8 = -32i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 5i8;
    let mut i8_7: i8 = -114i8;
    let mut i8_8: i8 = -42i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 16u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 98u8;
    let mut u8_2: u8 = 71u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = -68i32;
    let mut i64_1: i64 = -91i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut i64_2: i64 = -4i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i8_9: i8 = -21i8;
    let mut i8_10: i8 = -30i8;
    let mut i8_11: i8 = -43i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut date_0_ref_0: &crate::date::Date = &mut date_0;
    let mut f32_0: f32 = 115.551029f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i8_12: i8 = -96i8;
    let mut i8_13: i8 = -122i8;
    let mut i8_14: i8 = 21i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_3: i64 = -149i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i128_0: i128 = -141i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_4: i64 = -86i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut i64_5: i64 = 95i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::minutes(i64_5);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_10: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut f64_0: f64 = -34.946590f64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_11, duration_10);
    let mut i8_15: i8 = -48i8;
    let mut i8_16: i8 = -1i8;
    let mut i8_17: i8 = -82i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_18: i8 = -72i8;
    let mut i8_19: i8 = 62i8;
    let mut i8_20: i8 = -60i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut u32_1: u32 = 25u32;
    let mut u8_3: u8 = 83u8;
    let mut u8_4: u8 = 37u8;
    let mut u8_5: u8 = 22u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_6: i64 = 68i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut f32_1: f32 = -42.168155f32;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_14, duration_13);
    let mut i8_21: i8 = -100i8;
    let mut i8_22: i8 = 38i8;
    let mut i8_23: i8 = -19i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut f32_2: f32 = 3.837842f32;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::abs(duration_16);
    let mut duration_18: std::time::Duration = crate::duration::Duration::abs_std(duration_17);
    let mut i64_7: i64 = -161i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::abs(duration_19);
    let mut i64_8: i64 = 36i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_8);
    let mut duration_22: std::time::Duration = crate::duration::Duration::abs_std(duration_21);
    let mut i64_9: i64 = -196i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_9);
    let mut i8_24: i8 = 20i8;
    let mut i8_25: i8 = 22i8;
    let mut i8_26: i8 = 62i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i8_27: i8 = 11i8;
    let mut i8_28: i8 = 110i8;
    let mut i8_29: i8 = 45i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_4, utcoffset_9);
    let mut i32_1: i32 = crate::offset_date_time::OffsetDateTime::to_julian_day(offsetdatetime_5);
    panic!("From RustyUnit with love");
}
}