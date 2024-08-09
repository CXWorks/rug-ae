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

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_8499() {
//    rusty_monitor::set_test_id(8499);
    let mut i64_0: i64 = 1000000i64;
    let mut i64_1: i64 = 12i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds(i64_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_1);
    let mut i8_0: i8 = 23i8;
    let mut i8_1: i8 = -69i8;
    let mut i8_2: i8 = 2i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 76i32;
    let mut i64_2: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_2);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_1, i32_0);
    let mut duration_3: std::time::Duration = crate::duration::Duration::abs_std(duration_2);
    let mut i64_3: i64 = 9223372036854775807i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_3);
    let mut duration_5: std::time::Duration = crate::duration::Duration::abs_std(duration_4);
    let mut i8_3: i8 = 0i8;
    let mut i8_4: i8 = 127i8;
    let mut i8_5: i8 = 0i8;
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_5, i8_4, i8_3);
    let mut i32_1: i32 = 20i32;
    let mut i64_4: i64 = -238i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::new(i64_4, i32_1);
    let mut i8_6: i8 = 5i8;
    let mut i8_7: i8 = 3i8;
    let mut i8_8: i8 = -64i8;
    let mut utcoffset_2: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_8, i8_7, i8_6);
    let mut i64_5: i64 = 1i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::microseconds(i64_5);
    let mut i64_6: i64 = 3600i64;
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::weeks(i64_6);
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 28u8;
    let mut u8_1: u8 = 31u8;
    let mut u8_2: u8 = 11u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_2: i32 = 116i32;
    let mut i64_7: i64 = 253402300799i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::new(i64_7, i32_2);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::abs(duration_9);
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i32_3: i32 = 139i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::saturating_add(duration_10, duration_11);
//    panic!("From RustyUnit with love");
}
}