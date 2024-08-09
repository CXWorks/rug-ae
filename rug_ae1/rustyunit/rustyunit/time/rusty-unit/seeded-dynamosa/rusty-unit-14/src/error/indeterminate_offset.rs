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
#[timeout(30000)]fn rusty_test_6882() {
//    rusty_monitor::set_test_id(6882);
    let mut i32_0: i32 = 268i32;
    let mut i64_0: i64 = 1i64;
    let mut i32_1: i32 = 398i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut u32_0: u32 = 10u32;
    let mut u8_0: u8 = 53u8;
    let mut u8_1: u8 = 5u8;
    let mut u8_2: u8 = 5u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut f64_0: f64 = 0.000000f64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i32_2: i32 = 43i32;
    let mut date_1: crate::date::Date = crate::date::Date {value: i32_2};
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i64_1: i64 = 1i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut i32_3: i32 = 336i32;
    let mut date_2: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_2);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_0);
    let mut u16_0: u16 = 60u16;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_0, u16_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_1};
    let mut i32_4: i32 = 26i32;
    let mut i64_2: i64 = 2440588i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut i32_5: i32 = 12i32;
    let mut date_4: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_5);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_4: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut i64_3: i64 = -13i64;
    let mut i64_4: i64 = 24i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut i8_0: i8 = 24i8;
    let mut i8_1: i8 = 1i8;
    let mut i8_2: i8 = 127i8;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_2, i8_1, i8_0);
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_6, i32_4);
    let mut padding_0: time::Padding = crate::time::Padding::Optimize;
//    panic!("From RustyUnit with love");
}
}