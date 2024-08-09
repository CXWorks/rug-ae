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
#[timeout(30000)]fn rusty_test_6728() {
//    rusty_monitor::set_test_id(6728);
    let mut i64_0: i64 = 1i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::hours(i64_0);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_0);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i64_1: i64 = 0i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_1);
    let mut f64_0: f64 = 4794699203894837248.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_2, duration_2);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_0: i32 = 16i32;
    let mut i64_2: i64 = 3600i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_0);
    let mut i64_3: i64 = 12i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_3);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_4);
    let mut date_0: crate::date::Date = crate::offset_date_time::OffsetDateTime::date(offsetdatetime_5);
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_3);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Thursday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::previous(weekday_4);
    let mut weekday_6: weekday::Weekday = crate::weekday::Weekday::Tuesday;
    let mut weekday_7: weekday::Weekday = crate::weekday::Weekday::Monday;
    let mut weekday_8: weekday::Weekday = crate::weekday::Weekday::previous(weekday_7);
    let mut weekday_9: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u8_0: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_9);
    let mut u8_1: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_8);
    let mut u8_2: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_6);
    let mut u8_3: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_5);
    let mut u8_4: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_3);
    let mut u8_5: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_2);
    let mut u8_6: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_1);
    let mut u8_7: u8 = crate::weekday::Weekday::number_days_from_monday(weekday_0);
//    panic!("From RustyUnit with love");
}
}