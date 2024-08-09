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
fn rusty_test_2012() {
    rusty_monitor::set_test_id(2012);
    let mut i64_0: i64 = 29i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::weeks(i64_0);
    let mut i32_0: i32 = -73i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut i8_0: i8 = -38i8;
    let mut i8_1: i8 = -2i8;
    let mut i8_2: i8 = 76i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_1: i32 = 112i32;
    let mut i32_2: i32 = -197i32;
    let mut i64_1: i64 = 139i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_1);
    let mut i32_3: i32 = -4i32;
    let mut i64_2: i64 = -78i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_2, i32_3);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i128_0: i128 = 119i128;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_3: i64 = 12i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i8_3: i8 = -35i8;
    let mut i8_4: i8 = 8i8;
    let mut i8_5: i8 = 65i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i64_4: i64 = 89i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_5: i64 = -46i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::hours(i64_5);
    let mut u32_0: u32 = 62u32;
    let mut u8_0: u8 = 72u8;
    let mut u8_1: u8 = 26u8;
    let mut u8_2: u8 = 17u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f32_0: f32 = 79.548074f32;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut i64_6: i64 = -41i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_6);
    let mut i128_1: i128 = -2i128;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i32_4: i32 = -205i32;
    let mut i64_7: i64 = -15i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_7, i32_4);
    let mut duration_14: std::time::Duration = crate::duration::Duration::abs_std(duration_13);
    let mut u32_1: u32 = 31u32;
    let mut u8_3: u8 = 33u8;
    let mut u8_4: u8 = 89u8;
    let mut u8_5: u8 = 37u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_5: i32 = 16i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut date_2: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_0);
    panic!("From RustyUnit with love");
}
}