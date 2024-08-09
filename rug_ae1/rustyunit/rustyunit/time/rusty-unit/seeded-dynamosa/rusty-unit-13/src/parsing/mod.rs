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
#[timeout(30000)]fn rusty_test_6970() {
//    rusty_monitor::set_test_id(6970);
    let mut i32_0: i32 = 54i32;
    let mut f64_0: f64 = 4696837146684686336.000000f64;
    let mut u32_0: u32 = 1000000u32;
    let mut u8_0: u8 = 53u8;
    let mut u8_1: u8 = 1u8;
    let mut u16_0: u16 = 366u16;
    let mut i32_1: i32 = 364i32;
    let mut date_0: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_1, u16_0);
    let mut i8_0: i8 = 2i8;
    let mut i8_1: i8 = 3i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_0, utcoffset_0);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut i64_0: i64 = 2440588i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i8_3: i8 = 3i8;
    let mut i8_4: i8 = 6i8;
    let mut i8_5: i8 = 4i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = 4i8;
    let mut i8_7: i8 = 2i8;
    let mut i8_8: i8 = 6i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_1: i64 = 2147483647i64;
    let mut i8_9: i8 = 11i8;
    let mut i8_10: i8 = 0i8;
    let mut i64_2: i64 = 1000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut f64_1: f64 = 4607182418800017408.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_0, duration_1);
    let mut i8_11: i8 = 3i8;
    let mut i8_12: i8 = 1i8;
    let mut i8_13: i8 = 2i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_12, i8_9, i8_11);
    let mut i8_14: i8 = 0i8;
    let mut i8_15: i8 = 60i8;
    let mut i8_16: i8 = 59i8;
    let mut i8_17: i8 = 5i8;
    let mut i32_2: i32 = 376i32;
    let mut i64_3: i64 = 1i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut u8_2: u8 = 0u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_0, u8_1, u32_0);
    let mut i8_18: i8 = 2i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_10);
    let mut i8_19: i8 = 43i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_15, i8_16, i8_19);
    let mut i32_3: i32 = 392i32;
    let mut i64_4: i64 = 60i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_3, i32_3);
    let mut i8_20: i8 = 3i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_18, i8_17, i8_20);
    let mut i64_5: i64 = 1000000i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_6, duration_7);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_0);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_div(duration_5, i32_0);
//    panic!("From RustyUnit with love");
}
}