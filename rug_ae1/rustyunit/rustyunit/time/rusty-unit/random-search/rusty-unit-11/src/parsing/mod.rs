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
fn rusty_test_4763() {
    rusty_monitor::set_test_id(4763);
    let mut i32_0: i32 = -61i32;
    let mut i64_0: i64 = 89i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut i128_0: i128 = -133i128;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i8_0: i8 = 31i8;
    let mut i8_1: i8 = 8i8;
    let mut i8_2: i8 = 73i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut u32_0: u32 = 34u32;
    let mut u8_0: u8 = 92u8;
    let mut u8_1: u8 = 27u8;
    let mut u8_2: u8 = 18u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 2i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_0, time_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime {utc_datetime: primitivedatetime_0, offset: utcoffset_0};
    let mut i64_1: i64 = -8i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_1);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_2);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_2);
    let mut i8_3: i8 = -63i8;
    let mut i8_4: i8 = -21i8;
    let mut i8_5: i8 = -41i8;
    let mut i64_2: i64 = 30i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i8_6: i8 = -100i8;
    let mut i8_7: i8 = 88i8;
    let mut i8_8: i8 = -62i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_1: u32 = 95u32;
    let mut u8_3: u8 = 90u8;
    let mut u8_4: u8 = 99u8;
    let mut u8_5: u8 = 76u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i64_3: i64 = 104i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::days(i64_3);
    let mut i64_4: i64 = 65i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut i8_9: i8 = -7i8;
    let mut i8_10: i8 = 6i8;
    let mut i8_11: i8 = -42i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::abs(duration_7);
    let mut i64_5: i64 = -72i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_5);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::abs(duration_9);
    let mut duration_11: std::time::Duration = crate::duration::Duration::abs_std(duration_10);
    let mut i64_6: i64 = 27i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::microseconds(i64_6);
    let mut i8_12: i8 = -36i8;
    let mut i8_13: i8 = -72i8;
    let mut i8_14: i8 = 7i8;
    let mut i8_15: i8 = -42i8;
    let mut i8_16: i8 = 5i8;
    let mut i8_17: i8 = 69i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut i64_7: i64 = 10i64;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::weeks(i64_7);
    let mut duration_14: std::time::Duration = crate::duration::Duration::abs_std(duration_13);
    let mut u32_2: u32 = 39u32;
    let mut u8_6: u8 = 43u8;
    let mut u8_7: u8 = 0u8;
    let mut u8_8: u8 = 83u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i64_8: i64 = -22i64;
    let mut duration_15: crate::duration::Duration = crate::duration::Duration::minutes(i64_8);
    let mut i64_9: i64 = -73i64;
    let mut duration_16: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_9);
    let mut i8_18: i8 = 4i8;
    let mut i8_19: i8 = 2i8;
    let mut i8_20: i8 = 23i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_17: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i64_10: i64 = -121i64;
    let mut duration_18: crate::duration::Duration = crate::duration::Duration::seconds(i64_10);
    let mut duration_19: std::time::Duration = crate::duration::Duration::abs_std(duration_18);
    let mut i64_11: i64 = -31i64;
    let mut duration_20: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_11);
    let mut u32_3: u32 = 96u32;
    let mut u8_9: u8 = 25u8;
    let mut u8_10: u8 = 78u8;
    let mut u8_11: u8 = 24u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i8_21: i8 = -17i8;
    let mut i8_22: i8 = 48i8;
    let mut i8_23: i8 = -8i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i128_1: i128 = 69i128;
    let mut duration_21: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_1);
    let mut i64_12: i64 = -10i64;
    let mut duration_22: crate::duration::Duration = crate::duration::Duration::weeks(i64_12);
    let mut duration_23: std::time::Duration = crate::duration::Duration::abs_std(duration_22);
    let mut i8_24: i8 = -25i8;
    let mut i8_25: i8 = 5i8;
    let mut i8_26: i8 = 103i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut i64_13: i64 = -44i64;
    let mut duration_24: crate::duration::Duration = crate::duration::Duration::weeks(i64_13);
    let mut duration_25: std::time::Duration = crate::duration::Duration::abs_std(duration_24);
    let mut i32_2: i32 = -28i32;
    let mut i64_14: i64 = -11i64;
    let mut duration_26: crate::duration::Duration = crate::duration::Duration::days(i64_14);
    let mut duration_27: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_26, i32_2);
    let mut i64_15: i64 = -168i64;
    let mut duration_28: crate::duration::Duration = crate::duration::Duration::minutes(i64_15);
    let mut i32_3: i32 = 72i32;
    let mut i64_16: i64 = -136i64;
    let mut duration_29: crate::duration::Duration = crate::duration::Duration::new(i64_16, i32_3);
    let mut i8_27: i8 = 88i8;
    let mut i8_28: i8 = 63i8;
    let mut i8_29: i8 = -28i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut i8_30: i8 = 34i8;
    let mut i8_31: i8 = 78i8;
    let mut i8_32: i8 = -8i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut f32_0: f32 = 273.255520f32;
    let mut duration_30: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_31: std::time::Duration = crate::duration::Duration::abs_std(duration_30);
    let mut i128_2: i128 = 52i128;
    let mut duration_32: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_2);
    let mut i32_4: i32 = 34i32;
    let mut i64_17: i64 = -84i64;
    let mut duration_33: crate::duration::Duration = crate::duration::Duration::new(i64_17, i32_4);
    let mut i64_18: i64 = 131i64;
    let mut duration_34: crate::duration::Duration = crate::duration::Duration::microseconds(i64_18);
    let mut f64_0: f64 = 317.833620f64;
    let mut duration_35: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut duration_36: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_35, duration_34);
    let mut i64_19: i64 = -14i64;
    let mut duration_37: crate::duration::Duration = crate::duration::Duration::hours(i64_19);
    let mut duration_38: std::time::Duration = crate::duration::Duration::abs_std(duration_37);
    let mut i64_20: i64 = -21i64;
    let mut duration_39: crate::duration::Duration = crate::duration::Duration::weeks(i64_20);
    let mut u32_4: u32 = 90u32;
    let mut u8_12: u8 = 54u8;
    let mut u8_13: u8 = 99u8;
    let mut u8_14: u8 = 82u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i8_33: i8 = -47i8;
    let mut i8_34: i8 = -56i8;
    let mut i8_35: i8 = -110i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut i32_5: i32 = -1i32;
    let mut i64_21: i64 = 104i64;
    let mut duration_40: crate::duration::Duration = crate::duration::Duration::new(i64_21, i32_5);
    let mut i8_36: i8 = -88i8;
    let mut i8_37: i8 = 29i8;
    let mut i8_38: i8 = -113i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut f64_1: f64 = -97.390305f64;
    let mut duration_41: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_1);
    let mut i64_22: i64 = -127i64;
    let mut duration_42: crate::duration::Duration = crate::duration::Duration::minutes(i64_22);
    let mut duration_43: std::time::Duration = crate::duration::Duration::abs_std(duration_42);
    let mut u32_5: u32 = 21u32;
    let mut u8_15: u8 = 68u8;
    let mut u8_16: u8 = 7u8;
    let mut u8_17: u8 = 22u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i32_6: i32 = 8i32;
    let mut i64_23: i64 = 34i64;
    let mut duration_44: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_23, i32_6);
    let mut i32_7: i32 = 56i32;
    let mut i64_24: i64 = 116i64;
    let mut duration_45: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_24);
    let mut duration_46: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_45, i32_7);
    let mut i8_39: i8 = -126i8;
    let mut i8_40: i8 = 40i8;
    let mut i8_41: i8 = -102i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut instant_2: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_47: crate::duration::Duration = crate::instant::Instant::elapsed(instant_2);
    let mut f32_1: f32 = 65.594414f32;
    let mut duration_48: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_1);
    let mut duration_49: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_48, duration_47);
    let mut duration_50: std::time::Duration = crate::duration::Duration::abs_std(duration_49);
    panic!("From RustyUnit with love");
}
}