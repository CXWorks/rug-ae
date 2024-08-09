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
fn rusty_test_4524() {
    rusty_monitor::set_test_id(4524);
    let mut i64_0: i64 = 66i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i64_1: i64 = 241i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::weeks(i64_1);
    let mut f64_0: f64 = 67.996356f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_2, duration_1);
    let mut i32_0: i32 = -91i32;
    let mut i64_2: i64 = -135i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut i8_0: i8 = 88i8;
    let mut i8_1: i8 = -18i8;
    let mut i8_2: i8 = 14i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut f64_1: f64 = 40.498500f64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut u32_0: u32 = 58u32;
    let mut u8_0: u8 = 96u8;
    let mut u8_1: u8 = 21u8;
    let mut u8_2: u8 = 94u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = -80i32;
    let mut i64_3: i64 = 9i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_6, i32_1);
    let mut i32_2: i32 = -80i32;
    let mut i64_4: i64 = -245i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_8, i32_2);
    let mut duration_10: std::time::Duration = crate::duration::Duration::abs_std(duration_9);
    let mut i8_3: i8 = -24i8;
    let mut i8_4: i8 = 114i8;
    let mut i8_5: i8 = -5i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_5: i64 = 109i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut duration_12: std::time::Duration = crate::duration::Duration::abs_std(duration_11);
    let mut i64_6: i64 = 97i64;
    let mut i64_7: i64 = 208i64;
    let mut i8_6: i8 = 3i8;
    let mut u32_1: u32 = 19u32;
    let mut u8_3: u8 = 9u8;
    let mut u8_4: u8 = 55u8;
    let mut u8_5: u8 = 17u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_7: i8 = 14i8;
    let mut i8_8: i8 = 68i8;
    let mut i8_9: i8 = -109i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_9, i8_8, i8_7);
    let mut i128_0: i128 = 45i128;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u32_2: u32 = 48u32;
    let mut u8_6: u8 = 71u8;
    let mut u8_7: u8 = 98u8;
    let mut u8_8: u8 = 20u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_14: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u16_0: u16 = 27u16;
    let mut i32_3: i32 = 95i32;
    let mut date_1: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_14);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_2, time: time_2};
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_13);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_1, offset: utcoffset_2};
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_time(offsetdatetime_2, time_1);
    let mut i32_4: i32 = 9i32;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_4, i32_4);
    let mut duration_16: std::time::Duration = crate::duration::Duration::abs_std(duration_15);
    let mut i64_8: i64 = -54i64;
    let mut duration_17: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_8);
    let mut i8_10: i8 = 23i8;
    let mut i8_11: i8 = -94i8;
    let mut i8_12: i8 = 46i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_12, i8_11, i8_10);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_4, utcoffset_3);
    let mut date_3: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_17);
    let mut time_3: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_3);
    let mut u32_3: u32 = 25u32;
    let mut u8_9: u8 = 84u8;
    let mut u8_10: u8 = 60u8;
    let mut u8_11: u8 = 7u8;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut duration_19: crate::duration::Duration = crate::duration::Duration::abs(duration_18);
    let mut u16_1: u16 = 15u16;
    let mut i32_5: i32 = -252i32;
    let mut date_4: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_5, u16_1);
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_19);
    let mut i32_6: i32 = -157i32;
    let mut date_6: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_6);
    let mut i64_9: i64 = -85i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::minutes(i64_9);
    let mut duration_21: std::time::Duration = crate::duration::Duration::abs_std(duration_20);
    let mut i8_13: i8 = -53i8;
    let mut i8_14: i8 = -35i8;
    let mut i8_15: i8 = 65i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_15, i8_14, i8_13);
    let mut i8_16: i8 = -64i8;
    let mut i8_17: i8 = 117i8;
    let mut i8_18: i8 = 11i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_18, i8_17, i8_16);
    let mut i128_1: i128 = -55i128;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_6, duration_22);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_7: i32 = -44i32;
    let mut i64_10: i64 = -40i64;
    let mut duration_23: crate::duration::Duration = crate::duration::Duration::new(i64_10, i32_7);
    let mut i64_11: i64 = 48i64;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::hours(i64_11);
    let mut duration_25: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_24, duration_23);
    let mut i64_12: i64 = 47i64;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_12);
    let mut i128_2: i128 = -70i128;
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut duration_28: std::time::Duration = crate::duration::Duration::abs_std(duration_27);
    let mut i64_13: i64 = -97i64;
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::minutes(i64_13);
    let mut i32_8: i32 = 63i32;
    let mut i64_14: i64 = -49i64;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::new(i64_14, i32_8);
    let mut i32_9: i32 = -100i32;
    let mut i64_15: i64 = -50i64;
    let mut duration_31: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_15, i32_9);
    let mut i32_10: i32 = 47i32;
    let mut i64_16: i64 = 34i64;
    let mut i32_11: i32 = 137i32;
    let mut i64_17: i64 = 41i64;
    let mut duration_32: crate::duration::Duration = crate::duration::Duration::days(i64_17);
    let mut i8_19: i8 = 1i8;
    let mut i8_20: i8 = -31i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_6);
    let mut duration_33: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_7);
    let mut duration_34: std::time::Duration = crate::duration::Duration::abs_std(duration_33);
    let mut i32_12: i32 = 177i32;
    let mut date_7: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_12);
    let mut i64_18: i64 = -5i64;
    let mut duration_35: crate::duration::Duration = crate::duration::Duration::weeks(i64_18);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_32, i32_11);
    let mut duration_36: crate::duration::Duration = crate::duration::Duration::new(i64_16, i32_10);
    let mut i64_19: i64 = crate::duration::Duration::whole_hours(duration_31);
    let mut u8_12: u8 = crate::date::Date::iso_week(date_6);
    let mut result_0: std::result::Result<crate::primitive_date_time::PrimitiveDateTime, crate::error::component_range::ComponentRange> = crate::date::Date::with_hms_nano(date_5, u8_11, u8_10, u8_9, u32_3);
    let mut tuple_0: (bool, crate::time::Time) = crate::time::Time::adjusting_add_std(time_3, duration_16);
    panic!("From RustyUnit with love");
}
}