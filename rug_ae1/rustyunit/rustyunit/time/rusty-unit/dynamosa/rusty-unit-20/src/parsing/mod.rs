//! Parsing for various types.

pub(crate) mod combinator;
pub(crate) mod component;
pub(crate) mod parsable;
mod parsed;
pub(crate) mod shim;

pub use self::parsable::Parsable;
pub use self::parsed::Parsed;

/// An item that has been parsed. Represented as a `(remaining, value)` pair.
#[derive(Debug)]
pub(crate) struct ParsedItem<'a, T>(pub(crate) &'a [u8], pub(crate) T);

impl<'a, T> ParsedItem<'a, T> {
    /// Map the value to a new value, preserving the remaining input.
    pub(crate) fn map<U>(self, f: impl FnOnce(T) -> U) -> ParsedItem<'a, U> {
        ParsedItem(self.0, f(self.1))
    }

    /// Map the value to a new, optional value, preserving the remaining input.
    pub(crate) fn flat_map<U>(self, f: impl FnOnce(T) -> Option<U>) -> Option<ParsedItem<'a, U>> {
        Some(ParsedItem(self.0, f(self.1)?))
    }

    /// Map the value to a new, optional value, preserving the remaining input.
    pub(crate) fn flat_map_res<U, V>(
        self,
        f: impl FnOnce(T) -> Result<U, V>,
    ) -> Result<ParsedItem<'a, U>, V> {
        Ok(ParsedItem(self.0, f(self.1)?))
    }

    /// Assign the stored value to the provided target. The remaining input is returned.
    #[must_use = "this returns the remaining input"]
    pub(crate) fn assign_value_to(self, target: &mut Option<T>) -> &'a [u8] {
        *target = Some(self.1);
        self.0
    }
}

impl<'a> ParsedItem<'a, ()> {
    /// Discard the unit value, returning the remaining input.
    #[must_use = "this returns the remaining input"]
    pub(crate) const fn into_inner(self) -> &'a [u8] {
        self.0
    }
}

impl<'a> ParsedItem<'a, Option<()>> {
    /// Discard the potential unit value, returning the remaining input.
    #[must_use = "this returns the remaining input"]
    pub(crate) const fn into_inner(self) -> &'a [u8] {
        self.0
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5292() {
    rusty_monitor::set_test_id(5292);
    let mut i64_0: i64 = 191i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut weekday_0: weekday::Weekday = crate::offset_date_time::OffsetDateTime::weekday(offsetdatetime_1);
    let mut u16_0: u16 = 99u16;
    let mut i64_1: i64 = -79i64;
    let mut i32_0: i32 = 87i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut u16_1: u16 = 69u16;
    let mut i32_1: i32 = -12i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_1);
    let mut i32_2: i32 = -2i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_2};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_1);
    let mut i8_0: i8 = 58i8;
    let mut i8_1: i8 = -63i8;
    let mut i8_2: i8 = -118i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_3: i32 = -164i32;
    let mut date_3: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_offset(primitivedatetime_2, utcoffset_0);
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date_time(offsetdatetime_2, primitivedatetime_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_4: i32 = 17i32;
    let mut i64_2: i64 = -13i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_4);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_4: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut u16_2: u16 = 80u16;
    let mut u16_3: u16 = 14u16;
    let mut i32_5: i32 = 128i32;
    let mut i64_3: i64 = -163i64;
    let mut f64_0: f64 = 52.531481f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_0: u32 = 39u32;
    let mut u8_0: u8 = 97u8;
    let mut u8_1: u8 = 17u8;
    let mut u8_2: u8 = 74u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut date_5: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_3);
    let mut i32_6: i32 = 100i32;
    let mut date_6: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut u32_1: u32 = 79u32;
    let mut u8_3: u8 = 11u8;
    let mut u8_4: u8 = 60u8;
    let mut u8_5: u8 = 22u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_3: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i8_3: i8 = 24i8;
    let mut i8_4: i8 = -87i8;
    let mut i8_5: i8 = 63i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_5, utcoffset_1);
    let mut date_7: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_6);
    let mut date_8: crate::date::Date = crate::date::Date::saturating_sub(date_7, duration_3);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_8, time: time_2};
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_3, date_6);
    let mut i32_7: i32 = 88i32;
    let mut i64_4: i64 = -41i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut u32_2: u32 = 12u32;
    let mut u8_6: u8 = 51u8;
    let mut u8_7: u8 = 16u8;
    let mut u8_8: u8 = 15u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut f64_1: f64 = 152.695896f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i32_8: i32 = -24i32;
    let mut date_9: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_8);
    let mut date_10: crate::date::Date = crate::date::Date::saturating_add(date_9, duration_5);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_10, time: time_3};
    let mut primitivedatetime_6: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_5, duration_4);
    let mut i8_6: i8 = 84i8;
    let mut i8_7: i8 = 3i8;
    let mut i8_8: i8 = -67i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_5: i64 = 109i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_5);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_7, duration_6);
    let mut i64_6: i64 = 61i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::seconds(i64_6);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::abs(duration_9);
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_7, duration_10);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_8);
    let mut u16_4: u16 = 70u16;
    let mut i32_9: i32 = -37i32;
    let mut date_11: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_9, u16_4);
    let mut primitivedatetime_7: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_11, time_4);
    let mut primitivedatetime_8: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_7, duration_8);
    let mut i8_9: i8 = 80i8;
    let mut i8_10: i8 = 45i8;
    let mut i8_11: i8 = 74i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i64_7: i64 = -125i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_7);
    let mut u32_3: u32 = 16u32;
    let mut u8_9: u8 = 36u8;
    let mut u8_10: u8 = 77u8;
    let mut u8_11: u8 = 23u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i32_10: i32 = -105i32;
    let mut date_12: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_10);
    let mut primitivedatetime_9: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_12, time_5);
    let mut primitivedatetime_10: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_9, duration_11);
    let mut i64_8: i64 = -74i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds(i64_8);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut i8_12: i8 = 17i8;
    let mut i8_13: i8 = 91i8;
    let mut i8_14: i8 = 57i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_4: u32 = 33u32;
    let mut u8_12: u8 = 85u8;
    let mut u8_13: u8 = 35u8;
    let mut u8_14: u8 = 64u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i32_11: i32 = -189i32;
    let mut date_13: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_11, u16_2);
    let mut primitivedatetime_11: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_13, time: time_6};
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_11, offset: utcoffset_4};
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut f64_2: f64 = -57.938415f64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_2);
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_15, duration_14);
    let mut i32_12: i32 = -3i32;
    let mut date_14: crate::date::Date = crate::date::Date {value: i32_12};
    let mut date_15: crate::date::Date = crate::date::Date::saturating_add(date_14, duration_16);
    let mut i64_9: i64 = 32i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::hours(i64_9);
    let mut u32_5: u32 = 25u32;
    let mut u8_15: u8 = 21u8;
    let mut u8_16: u8 = 54u8;
    let mut u8_17: u8 = 37u8;
    let mut time_7: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i32_13: i32 = 212i32;
    let mut date_16: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_7, u16_0);
    let mut date_17: crate::date::Date = crate::date::Date::saturating_sub(date_4, duration_12);
    let mut date_18: crate::date::Date = crate::date::Date {value: i32_13};
    let mut primitivedatetime_12: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_17);
    let mut tuple_0: (i32, month::Month, u8) = crate::primitive_date_time::PrimitiveDateTime::to_calendar_date(primitivedatetime_8);
    let mut u8_18: u8 = crate::date::Date::iso_week(date_18);
    let mut u8_19: u8 = crate::weekday::Weekday::number_from_sunday(weekday_0);
    let mut time_8: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_6);
    panic!("From RustyUnit with love");
}
}