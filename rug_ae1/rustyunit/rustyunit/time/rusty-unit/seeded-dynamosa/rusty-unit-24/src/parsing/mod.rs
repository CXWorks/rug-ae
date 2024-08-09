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
#[timeout(30000)]fn rusty_test_330() {
//    rusty_monitor::set_test_id(330);
    let mut i32_0: i32 = 207i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut month_0: month::Month = crate::date::Date::month(date_0);
    let mut i64_0: i64 = 3600i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i8_0: i8 = 59i8;
    let mut i8_1: i8 = 125i8;
    let mut i8_2: i8 = 24i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i64_1: i64 = 1000i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::hours(i64_1);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut i8_3: i8 = 2i8;
    let mut i8_4: i8 = 59i8;
    let mut i8_5: i8 = -33i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut u32_0: u32 = 100u32;
    let mut u8_0: u8 = 74u8;
    let mut u8_1: u8 = 2u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut month_1: month::Month = crate::month::Month::February;
    let mut month_2: month::Month = crate::month::Month::next(month_1);
    let mut month_3: month::Month = crate::month::Month::June;
    let mut month_4: month::Month = crate::month::Month::next(month_3);
    let mut month_5: month::Month = crate::month::Month::August;
    let mut month_6: month::Month = crate::month::Month::July;
    let mut month_7: month::Month = crate::month::Month::October;
    let mut month_8: month::Month = crate::month::Month::January;
    let mut month_9: month::Month = crate::month::Month::October;
    let mut month_10: month::Month = crate::month::Month::August;
    let mut month_11: month::Month = crate::month::Month::previous(month_10);
    let mut i32_1: i32 = 1000i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_1};
    let mut month_12: month::Month = crate::date::Date::month(date_1);
    let mut month_13: month::Month = crate::month::Month::February;
    let mut month_14: month::Month = crate::month::Month::previous(month_13);
    let mut month_15: month::Month = crate::month::Month::previous(month_12);
    let mut month_16: month::Month = crate::month::Month::previous(month_11);
    let mut month_17: month::Month = crate::month::Month::previous(month_9);
    let mut month_18: month::Month = crate::month::Month::previous(month_8);
    let mut month_19: month::Month = crate::month::Month::previous(month_7);
    let mut month_20: month::Month = crate::month::Month::previous(month_6);
    let mut month_21: month::Month = crate::month::Month::previous(month_5);
    let mut month_22: month::Month = crate::month::Month::previous(month_4);
    let mut month_23: month::Month = crate::month::Month::previous(month_2);
    let mut month_24: month::Month = crate::month::Month::previous(month_0);
//    panic!("From RustyUnit with love");
}
}