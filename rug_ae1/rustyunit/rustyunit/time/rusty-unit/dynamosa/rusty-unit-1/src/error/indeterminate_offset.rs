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
fn rusty_test_6229() {
    rusty_monitor::set_test_id(6229);
    let mut i128_0: i128 = -173i128;
    let mut i128_1: i128 = 86i128;
    let mut i64_0: i64 = 148i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::microseconds(i64_0);
    let mut i32_0: i32 = 100i32;
    let mut i64_1: i64 = -126i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_1, i32_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_0);
    let mut f32_0: f32 = -40.468597f32;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut u32_0: u32 = 50u32;
    let mut u8_0: u8 = 96u8;
    let mut u8_1: u8 = 78u8;
    let mut u8_2: u8 = 29u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_1: i32 = 43i32;
    let mut i64_2: i64 = 79i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut i64_3: i64 = 64i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::seconds(i64_3);
    let mut i32_2: i32 = 10i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_2);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_4);
    let mut i64_4: i64 = 40i64;
    let mut u16_0: u16 = 80u16;
    let mut i32_3: i32 = -56i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_3, u16_0);
    let mut u16_1: u16 = 77u16;
    let mut i32_4: i32 = 103i32;
    let mut date_3: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_4, u16_1);
    let mut i32_5: i32 = -235i32;
    let mut date_4: crate::date::Date = crate::date::Date {value: i32_5};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::midnight(date_4);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::replace_date(primitivedatetime_0, date_3);
    let mut month_0: month::Month = crate::month::Month::August;
    let mut month_0_ref_0: &month::Month = &mut month_0;
    let mut month_1: month::Month = crate::month::Month::April;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::microseconds(i64_4);
    let mut dateadjustment_0: util::DateAdjustment = crate::util::DateAdjustment::Next;
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::checked_add(date_1, duration_3);
    let mut i64_5: i64 = crate::offset_date_time::OffsetDateTime::unix_timestamp(offsetdatetime_1);
    let mut result_0: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_1);
    let mut result_1: std::result::Result<crate::offset_date_time::OffsetDateTime, crate::error::component_range::ComponentRange> = crate::offset_date_time::OffsetDateTime::from_unix_timestamp_nanos(i128_0);
    panic!("From RustyUnit with love");
}
}