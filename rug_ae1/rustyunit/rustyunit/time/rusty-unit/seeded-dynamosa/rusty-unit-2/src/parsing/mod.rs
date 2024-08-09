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
#[timeout(30000)]fn rusty_test_373() {
//    rusty_monitor::set_test_id(373);
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 59u8;
    let mut u8_1: u8 = 23u8;
    let mut u8_2: u8 = 0u8;
    let mut u32_1: u32 = 100u32;
    let mut u8_3: u8 = 52u8;
    let mut u8_4: u8 = 23u8;
    let mut u8_5: u8 = 42u8;
    let mut u32_2: u32 = 100000u32;
    let mut u8_6: u8 = 24u8;
    let mut u8_7: u8 = 53u8;
    let mut u8_8: u8 = 66u8;
    let mut u32_3: u32 = 999999999u32;
    let mut u8_9: u8 = 12u8;
    let mut u8_10: u8 = 4u8;
    let mut u8_11: u8 = 52u8;
    let mut u32_4: u32 = 10000000u32;
    let mut u8_12: u8 = 12u8;
    let mut u8_13: u8 = 6u8;
    let mut u8_14: u8 = 29u8;
    let mut u32_5: u32 = 1000000000u32;
    let mut u8_15: u8 = 59u8;
    let mut u8_16: u8 = 7u8;
    let mut u8_17: u8 = 12u8;
    let mut u32_6: u32 = 39u32;
    let mut u8_18: u8 = 30u8;
    let mut u8_19: u8 = 58u8;
    let mut u8_20: u8 = 30u8;
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_20, u8_19, u8_18, u32_6);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_17, u8_16, u8_15, u32_5);
    let mut result_2: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_14, u8_13, u8_12, u32_4);
    let mut result_3: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_11, u8_10, u8_9, u32_3);
    let mut result_4: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_8, u8_7, u8_6, u32_2);
    let mut result_5: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_5, u8_4, u8_3, u32_1);
    let mut result_6: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_micro(u8_2, u8_1, u8_0, u32_0);
//    panic!("From RustyUnit with love");
}
}