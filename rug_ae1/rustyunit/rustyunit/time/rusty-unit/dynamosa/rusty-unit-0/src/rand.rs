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
fn rusty_test_6435() {
    rusty_monitor::set_test_id(6435);
    let mut i8_0: i8 = -76i8;
    let mut i8_1: i8 = 11i8;
    let mut i8_2: i8 = 73i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_0: i64 = -75i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut u16_0: u16 = 28u16;
    let mut i32_0: i32 = 67i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_1, utcoffset_0);
    let mut i8_3: i8 = 101i8;
    let mut i8_4: i8 = 102i8;
    let mut i8_5: i8 = -40i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_1: i32 = -188i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut u32_0: u32 = 35u32;
    let mut u8_0: u8 = 3u8;
    let mut u8_1: u8 = 22u8;
    let mut u8_2: u8 = 38u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i128_0: i128 = 75i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u16_1: u16 = 92u16;
    let mut i32_2: i32 = -155i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_1);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_0};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_2, date_1);
    let mut i32_3: i32 = -98i32;
    let mut i64_1: i64 = -146i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_2);
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i32_4: i32 = -94i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_4};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_4);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_3, utcoffset_2);
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_4);
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut i32_5: i32 = -230i32;
    let mut i64_2: i64 = 50i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_5);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_5, duration_3);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_6);
    let mut i8_6: i8 = 120i8;
    let mut i8_7: i8 = 65i8;
    let mut i8_8: i8 = -91i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_3: i64 = -175i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::hours(i64_3);
    let mut i8_9: i8 = -75i8;
    let mut i8_10: i8 = 88i8;
    let mut i8_11: i8 = -89i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = 10i8;
    let mut i8_13: i8 = 36i8;
    let mut i8_14: i8 = 73i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut i64_4: i64 = 249i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::hours(i64_4);
    let mut f32_0: f32 = -157.828369f32;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_15: i8 = 9i8;
    let mut i8_16: i8 = 62i8;
    let mut i8_17: i8 = -28i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i8_18: i8 = 11i8;
    let mut i8_19: i8 = 24i8;
    let mut i8_20: i8 = -15i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i128_1: i128 = 75i128;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut u32_1: u32 = 99u32;
    let mut u8_3: u8 = 35u8;
    let mut u8_4: u8 = 27u8;
    let mut u8_5: u8 = 38u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_21: i8 = 48i8;
    let mut i8_22: i8 = 49i8;
    let mut i8_23: i8 = 117i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i32_6: i32 = -20i32;
    let mut i64_5: i64 = 9i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_8, i32_6);
    let mut i8_24: i8 = -20i8;
    let mut i8_25: i8 = -71i8;
    let mut i8_26: i8 = -13i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut u32_2: u32 = 2u32;
    let mut u8_6: u8 = 89u8;
    let mut u8_7: u8 = 21u8;
    let mut u8_8: u8 = 78u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_6: i64 = -28i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut i32_7: i32 = -113i32;
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_11: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_11, i32_7);
    let mut i64_7: i64 = 76i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut i64_8: i64 = 211i64;
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::microseconds(i64_8);
    let mut i32_8: i32 = -70i32;
    let mut date_6: crate::date::Date = crate::date::Date {value: i32_8};
    let mut date_7: crate::date::Date = crate::date::Date::saturating_add(date_6, duration_14);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_5);
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_5, duration_13);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_6);
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_7, duration_12);
    let mut i32_9: i32 = 20i32;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::next(weekday_0);
    let mut u32_3: u32 = 29u32;
    let mut u8_9: u8 = 2u8;
    let mut u8_10: u8 = 83u8;
    let mut u8_11: u8 = 9u8;
    let mut i64_9: i64 = 68i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::weeks(i64_9);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::abs(duration_15);
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_8: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_9);
    let mut date_9: crate::date::Date = crate::date::Date::saturating_add(date_8, duration_16);
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut f64_0: f64 = 106.341160f64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_10: i64 = 64i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::minutes(i64_10);
    let mut offsetdatetime_11: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_12: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_11, duration_18);
    let mut date_10: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_12);
    let mut date_11: crate::date::Date = crate::date::Date::saturating_add(date_10, duration_17);
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_11);
    let mut i64_11: i64 = -30i64;
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::weeks(i64_11);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_27: i8 = 39i8;
    let mut i8_28: i8 = 51i8;
    let mut i8_29: i8 = 91i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut f64_1: f64 = 64.196562f64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i8_30: i8 = 38i8;
    let mut i8_31: i8 = -21i8;
    let mut i8_32: i8 = 57i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut u32_4: u32 = 98u32;
    let mut u8_12: u8 = 45u8;
    let mut u8_13: u8 = 45u8;
    let mut u8_14: u8 = 37u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut u32_5: u32 = 21u32;
    let mut u8_15: u8 = 0u8;
    let mut u8_16: u8 = 59u8;
    let mut u8_17: u8 = 56u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i32_10: i32 = 18i32;
    let mut date_12: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_10);
    let mut primitivedatetime_8: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_12, time_4);
    let mut primitivedatetime_9: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_8, time_3);
    let mut offsetdatetime_13: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_9, utcoffset_11);
    let mut i64_12: i64 = -81i64;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_12);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut month_1_ref_0: &month::Month = &mut month_1;
    let mut bool_0: bool = crate::duration::Duration::is_zero(duration_21);
    let mut bool_1: bool = crate::utc_offset::UtcOffset::is_negative(utcoffset_10);
    let mut option_0: std::option::Option<crate::instant::Instant> = crate::instant::Instant::checked_add(instant_1, duration_19);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_9, u8_11, u8_10, u8_9, u32_3);
    let mut u8_18: u8 = crate::util::weeks_in_year(i32_9);
    let mut tuple_0: (u8, u8, u8, u32) = crate::offset_date_time::OffsetDateTime::to_hms_micro(offsetdatetime_8);
    panic!("From RustyUnit with love");
}
}