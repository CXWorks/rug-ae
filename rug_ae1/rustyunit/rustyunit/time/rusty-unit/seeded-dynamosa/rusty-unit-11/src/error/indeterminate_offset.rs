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
#[timeout(30000)]fn rusty_test_8033() {
//    rusty_monitor::set_test_id(8033);
    let mut i32_0: i32 = 229i32;
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Wednesday;
    let mut u8_0: u8 = 33u8;
    let mut i32_1: i32 = 54i32;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Sunday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::next(weekday_1);
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut u8_1: u8 = 0u8;
    let mut i64_0: i64 = 1000007i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::minutes(i64_0);
    let mut i32_2: i32 = -154i32;
    let mut u8_2: u8 = 24u8;
    let mut i32_3: i32 = 105i32;
    let mut date_0: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_3);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_0);
    let mut i64_1: i64 = -54i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_1);
    let mut duration_2: std::time::Duration = crate::duration::Duration::abs_std(duration_1);
    let mut f64_0: f64 = 4607182418800017408.000000f64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut i64_2: i64 = 24i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::microseconds(i64_2);
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::saturating_sub(duration_4, duration_3);
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_1, duration_5);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_2);
    let mut i64_3: i64 = 12i64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::weeks(i64_3);
    let mut i64_4: i64 = 59i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::seconds(i64_4);
    let mut duration_8: std::time::Duration = crate::duration::Duration::abs_std(duration_7);
    let mut i64_5: i64 = 1000000i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_5);
    let mut i32_4: i32 = 25i32;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::Saturday;
    let mut result_0: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_2, u8_2, weekday_4);
    let mut result_1: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_4, u8_1, weekday_3);
    let mut result_2: std::result::Result<crate::date::Date, crate::error::component_range::ComponentRange> = crate::date::Date::from_iso_week_date(i32_1, u8_0, weekday_0);
    let mut bool_0: bool = crate::util::is_leap_year(i32_0);
//    panic!("From RustyUnit with love");
}
}