//! Indeterminate offset

use core::convert::TryFrom;
use core::fmt;

use crate::error;

/// The system's UTC offset could not be determined at the given datetime.
#[cfg_attr(__time_03_docs, doc(cfg(feature = "local-offset")))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndeterminateOffset;

impl fmt::Display for IndeterminateOffset {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("The system's UTC offset could not be determined")
    }
}

#[cfg(feature = "std")]
impl std::error::Error for IndeterminateOffset {}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "local-offset")))]
impl From<IndeterminateOffset> for crate::Error {
    fn from(err: IndeterminateOffset) -> Self {
        Self::IndeterminateOffset(err)
    }
}

#[cfg_attr(__time_03_docs, doc(cfg(feature = "std")))]
impl TryFrom<crate::Error> for IndeterminateOffset {
    type Error = error::DifferentVariant;

    fn try_from(err: crate::Error) -> Result<Self, Self::Error> {
        match err {
            crate::Error::IndeterminateOffset(err) => Ok(err),
            _ => Err(error::DifferentVariant),
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
fn rusty_test_2901() {
    rusty_monitor::set_test_id(2901);
    let mut i64_0: i64 = 70i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_0);
    let mut i8_0: i8 = -20i8;
    let mut i8_1: i8 = -11i8;
    let mut i8_2: i8 = -8i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::to_offset(offsetdatetime_0, utcoffset_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_3: i8 = 38i8;
    let mut i8_4: i8 = 57i8;
    let mut i8_5: i8 = -46i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i8_6: i8 = -50i8;
    let mut i8_7: i8 = 60i8;
    let mut i8_8: i8 = -9i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut u32_0: u32 = 8u32;
    let mut u8_0: u8 = 93u8;
    let mut u8_1: u8 = 89u8;
    let mut u8_2: u8 = 1u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i8_9: i8 = 20i8;
    let mut i8_10: i8 = 98i8;
    let mut i8_11: i8 = 37i8;
    let mut utcoffset_3: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_11, i8_10, i8_9);
    let mut i8_12: i8 = -79i8;
    let mut i8_13: i8 = -44i8;
    let mut i8_14: i8 = -85i8;
    let mut utcoffset_4: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_14, i8_13, i8_12);
    let mut u32_1: u32 = 58u32;
    let mut u8_3: u8 = 41u8;
    let mut u8_4: u8 = 82u8;
    let mut u8_5: u8 = 23u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i8_15: i8 = 71i8;
    let mut i8_16: i8 = -25i8;
    let mut i8_17: i8 = 3i8;
    let mut utcoffset_5: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_17, i8_16, i8_15);
    let mut u32_2: u32 = 59u32;
    let mut u8_6: u8 = 87u8;
    let mut u8_7: u8 = 83u8;
    let mut u8_8: u8 = 6u8;
    let mut time_2: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_8, u8_7, u8_6, u32_2);
    let mut i8_18: i8 = -34i8;
    let mut i8_19: i8 = -59i8;
    let mut i8_20: i8 = 6i8;
    let mut utcoffset_6: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_20, i8_19, i8_18);
    let mut i8_21: i8 = 28i8;
    let mut i8_22: i8 = -3i8;
    let mut i8_23: i8 = 127i8;
    let mut utcoffset_7: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_23, i8_22, i8_21);
    let mut i8_24: i8 = 6i8;
    let mut i8_25: i8 = -56i8;
    let mut i8_26: i8 = -67i8;
    let mut utcoffset_8: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_26, i8_25, i8_24);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_offset(offsetdatetime_2, utcoffset_8);
    let mut date_1: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_3);
    let mut i8_27: i8 = -102i8;
    let mut i8_28: i8 = -9i8;
    let mut i8_29: i8 = -40i8;
    let mut utcoffset_9: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_29, i8_28, i8_27);
    let mut u32_3: u32 = 20u32;
    let mut u8_9: u8 = 39u8;
    let mut u8_10: u8 = 84u8;
    let mut u8_11: u8 = 59u8;
    let mut time_3: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_11, u8_10, u8_9, u32_3);
    let mut i8_30: i8 = 1i8;
    let mut i8_31: i8 = 93i8;
    let mut i8_32: i8 = 115i8;
    let mut utcoffset_10: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_32, i8_31, i8_30);
    let mut i8_33: i8 = -12i8;
    let mut i8_34: i8 = 116i8;
    let mut i8_35: i8 = 3i8;
    let mut utcoffset_11: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_35, i8_34, i8_33);
    let mut u32_4: u32 = 32u32;
    let mut u8_12: u8 = 78u8;
    let mut u8_13: u8 = 76u8;
    let mut u8_14: u8 = 54u8;
    let mut time_4: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_14, u8_13, u8_12, u32_4);
    let mut i8_36: i8 = 36i8;
    let mut i8_37: i8 = 43i8;
    let mut i8_38: i8 = -67i8;
    let mut utcoffset_12: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_38, i8_37, i8_36);
    let mut i8_39: i8 = 42i8;
    let mut i8_40: i8 = -123i8;
    let mut i8_41: i8 = 24i8;
    let mut utcoffset_13: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_41, i8_40, i8_39);
    let mut i8_42: i8 = 29i8;
    let mut i8_43: i8 = -82i8;
    let mut i8_44: i8 = 34i8;
    let mut utcoffset_14: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_44, i8_43, i8_42);
    let mut i8_45: i8 = -49i8;
    let mut i8_46: i8 = -19i8;
    let mut i8_47: i8 = -37i8;
    let mut utcoffset_15: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_47, i8_46, i8_45);
    let mut u32_5: u32 = 2u32;
    let mut u8_15: u8 = 64u8;
    let mut u8_16: u8 = 26u8;
    let mut u8_17: u8 = 68u8;
    let mut time_5: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_17, u8_16, u8_15, u32_5);
    let mut i8_48: i8 = -44i8;
    let mut i8_49: i8 = -44i8;
    let mut i8_50: i8 = -97i8;
    let mut utcoffset_16: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_50, i8_49, i8_48);
    let mut i8_51: i8 = -57i8;
    let mut i8_52: i8 = 6i8;
    let mut i8_53: i8 = -45i8;
    let mut utcoffset_17: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_53, i8_52, i8_51);
    let mut u32_6: u32 = 4u32;
    let mut u8_18: u8 = 56u8;
    let mut u8_19: u8 = 29u8;
    let mut u8_20: u8 = 50u8;
    let mut time_6: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_20, u8_19, u8_18, u32_6);
    let mut i8_54: i8 = 100i8;
    let mut i8_55: i8 = 49i8;
    let mut i8_56: i8 = 57i8;
    let mut utcoffset_18: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_56, i8_55, i8_54);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut i8_57: i8 = -7i8;
    let mut i8_58: i8 = -31i8;
    let mut i8_59: i8 = 10i8;
    let mut utcoffset_19: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_59, i8_58, i8_57);
    let mut i32_0: i32 = 8i32;
    let mut i64_1: i64 = -54i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut i32_1: i32 = -27i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_sub(date_2, duration_1);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_3);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_19);
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::primitive_date_time::PrimitiveDateTime::assume_utc(primitivedatetime_1);
    let mut time_7: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_5);
    let mut time_7_ref_0: &mut crate::time::Time = &mut time_7;
    let mut i64_2: i64 = 32i64;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut u8_21: u8 = crate::weekday::Weekday::number_days_from_sunday(weekday_0);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::days(i64_2);
    panic!("From RustyUnit with love");
}
}