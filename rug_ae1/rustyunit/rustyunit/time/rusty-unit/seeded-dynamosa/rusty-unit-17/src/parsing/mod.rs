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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_9086() {
//    rusty_monitor::set_test_id(9086);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut i32_0: i32 = 224i32;
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i8_0: i8 = 1i8;
    let mut i8_1: i8 = 5i8;
    let mut i8_2: i8 = 93i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i32_1: i32 = 342i32;
    let mut f32_1: f32 = -6.232377f32;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_1);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_2);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_0);
    let mut primitivedatetime_1_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_1;
    let mut i8_3: i8 = 60i8;
    let mut i8_4: i8 = 2i8;
    let mut i8_5: i8 = 0i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut f32_2: f32 = 1315859240.000000f32;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_2);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i32_2: i32 = 7i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_1, duration_4);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_2, duration_3);
    let mut i32_3: i32 = 9i32;
    let mut i128_0: i128 = 1000000i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut u16_0: u16 = 60u16;
    let mut i32_4: i32 = 48i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_0);
    let mut date_4: crate::date::Date = crate::date::Date::saturating_sub(date_3, duration_5);
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i64_1: i64 = 0i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_7, duration_6);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i64_2: i64 = 1000i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::minutes(i64_2);
    let mut i64_3: i64 = 1000i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut i32_5: i32 = 1721425i32;
    let mut i64_4: i64 = 1000000i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_5);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_12, duration_11);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_4, duration_13);
    let mut date_5: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut date_6: crate::date::Date = crate::date::Date::saturating_sub(date_5, duration_10);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_6);
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_3);
    let mut primitivedatetime_5_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_5;
    let mut u8_0: u8 = crate::util::days_in_year_month(i32_0, month_0);
    let mut month_1: month::Month = crate::month::Month::September;
    let mut utcoffset_1_ref_0: &crate::utc_offset::UtcOffset = &mut utcoffset_1;
//    panic!("From RustyUnit with love");
}
}