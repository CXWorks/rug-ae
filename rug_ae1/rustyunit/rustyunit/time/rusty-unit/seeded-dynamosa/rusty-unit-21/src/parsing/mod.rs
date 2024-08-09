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
#[timeout(30000)]fn rusty_test_8721() {
//    rusty_monitor::set_test_id(8721);
    let mut i64_0: i64 = 1000000000i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut duration_1: std::time::Duration = crate::duration::Duration::abs_std(duration_0);
    let mut i64_1: i64 = 12i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i32_0: i32 = 76i32;
    let mut i64_2: i64 = 9223372036854775807i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_0);
    let mut i64_3: i64 = 3600i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::abs(duration_5);
    let mut u16_0: u16 = 10u16;
    let mut i32_1: i32 = 370i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_6);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_add(primitivedatetime_0, duration_4);
    let mut time_0: crate::time::Time = crate::primitive_date_time::PrimitiveDateTime::time(primitivedatetime_1);
    let mut i8_0: i8 = 0i8;
    let mut i8_1: i8 = -124i8;
    let mut i8_2: i8 = 6i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_4: i64 = 86400i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_4);
    let mut i32_2: i32 = 82i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_7);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_2, date_3);
    let mut time_2: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i64_5: i64 = 12i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_5);
    let mut i32_3: i32 = 128i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_3};
    let mut date_5: crate::date::Date = crate::date::Date::saturating_add(date_4, duration_8);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_5, time: time_2};
    let mut primitivedatetime_3: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_time(primitivedatetime_2, time_1);
    let mut primitivedatetime_3_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_3;
    let mut i8_3: i8 = 59i8;
    let mut i8_4: i8 = 3i8;
    let mut i8_5: i8 = 127i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 1000000u32;
    let mut u8_0: u8 = 8u8;
    let mut u8_1: u8 = 1u8;
    let mut u8_2: u8 = 6u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_4: i32 = 0i32;
    let mut i64_6: i64 = 60i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_4);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut date_6: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_4);
    let mut date_7: crate::date::Date = crate::date::Date::saturating_add(date_6, duration_9);
    let mut i64_7: i64 = 2147483647i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut i64_8: i64 = -242i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::days(i64_8);
    let mut i64_9: i64 = 60i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::days(i64_9);
    let mut i32_5: i32 = 167i32;
    let mut date_8: crate::date::Date = crate::date::Date {value: i32_5};
    let mut date_9: crate::date::Date = crate::date::Date::saturating_add(date_8, duration_12);
    let mut primitivedatetime_4: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_9);
    let mut i64_10: i64 = 0i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::hours(i64_10);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_5, duration_13);
    let mut time_4: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_6);
    let mut i32_6: i32 = -64i32;
    let mut date_10: crate::date::Date = crate::date::Date {value: i32_6};
    let mut primitivedatetime_5: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_10, time: time_4};
    let mut primitivedatetime_5_ref_0: &crate::primitive_date_time::PrimitiveDateTime = &mut primitivedatetime_5;
    let mut month_0: month::Month = crate::month::Month::February;
//    panic!("From RustyUnit with love");
}
}