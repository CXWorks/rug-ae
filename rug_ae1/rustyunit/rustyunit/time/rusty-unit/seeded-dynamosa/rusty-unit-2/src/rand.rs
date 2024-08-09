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
#[timeout(30000)]fn rusty_test_6916() {
//    rusty_monitor::set_test_id(6916);
    let mut month_0: month::Month = crate::month::Month::May;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut f32_0: f32 = 170.200620f32;
    let mut i8_0: i8 = 23i8;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut u8_0: u8 = 29u8;
    let mut i32_0: i32 = 5i32;
    let mut i64_0: i64 = 604800i64;
    let mut i64_1: i64 = 24i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut u32_0: u32 = 10000000u32;
    let mut u8_1: u8 = 0u8;
    let mut u8_2: u8 = 11u8;
    let mut u8_3: u8 = 11u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_3, u8_2, u8_1, u32_0);
    let mut i64_2: i64 = 160i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_2);
    let mut u16_0: u16 = 10u16;
    let mut i32_1: i32 = 400i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_add(date_0, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_1, time: time_0};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut i64_3: i64 = 604800i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut i128_0: i128 = 0i128;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_3, duration_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_4);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_2: i32 = 43i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i8_1: i8 = 127i8;
    let mut i64_4: i64 = 1000i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_2: i8 = 5i8;
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_3, i8_2, i8_1);
    let mut u32_1: u32 = 10u32;
    let mut u8_4: u8 = 52u8;
    let mut u8_5: u8 = 34u8;
    let mut u8_6: u8 = 0u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_4, u8_0, u8_5, u32_1);
    let mut i8_5: i8 = 59i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_0, i8_4);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_6);
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_calendar_date(i32_0, month_0, u8_6);
//    panic!("From RustyUnit with love");
}
}