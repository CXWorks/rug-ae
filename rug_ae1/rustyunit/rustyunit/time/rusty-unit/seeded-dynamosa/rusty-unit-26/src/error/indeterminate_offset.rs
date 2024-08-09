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
#[timeout(30000)]fn rusty_test_399() {
//    rusty_monitor::set_test_id(399);
    let mut i32_0: i32 = 3i32;
    let mut i32_1: i32 = 105i32;
    let mut i32_2: i32 = 128i32;
    let mut i32_3: i32 = -144i32;
    let mut i32_4: i32 = -118i32;
    let mut i32_5: i32 = 76i32;
    let mut i32_6: i32 = 178i32;
    let mut i32_7: i32 = 207i32;
    let mut i32_8: i32 = 325i32;
    let mut i32_9: i32 = 167i32;
    let mut i32_10: i32 = 365i32;
    let mut i32_11: i32 = 336i32;
    let mut i32_12: i32 = 376i32;
    let mut i32_13: i32 = -111i32;
    let mut i32_14: i32 = 189i32;
    let mut i32_15: i32 = 229i32;
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_15);
    let mut result_1: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_14);
    let mut result_2: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_13);
    let mut result_3: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_12);
    let mut result_4: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_11);
    let mut result_5: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_10);
    let mut result_6: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_9);
    let mut result_7: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_8);
    let mut result_8: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_7);
    let mut result_9: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_6);
    let mut result_10: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_5);
    let mut result_11: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_4);
    let mut result_12: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_3);
    let mut result_13: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_2);
    let mut result_14: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_1);
    let mut result_15: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_whole_seconds(i32_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_327() {
//    rusty_monitor::set_test_id(327);
    let mut weekday_0: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_1: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_2: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_3: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_4: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_5: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_6: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_7: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_8: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_9: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_10: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_11: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_12: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_13: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_14: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_15: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_16: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_17: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_18: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_19: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_20: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_21: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_22: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_23: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_24: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_25: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_26: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_27: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_28: weekday::Weekday = crate::weekday::Weekday::Friday;
    let mut weekday_29: weekday::Weekday = crate::weekday::Weekday::Friday;
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_497() {
//    rusty_monitor::set_test_id(497);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_6: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_7: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_8: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_9: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_10: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_11: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_12: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_13: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_14: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_15: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_16: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_17: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_18: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_19: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_20: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_21: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_22: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_23: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_24: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_25: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_26: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_27: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_28: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_29: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_308() {
//    rusty_monitor::set_test_id(308);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 86400i64;
    let mut i64_1: i64 = -125i64;
    let mut i64_2: i64 = 3600i64;
    let mut str_0: &str = "January";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut bool_1: bool = true;
    let mut i64_3: i64 = 1000000000i64;
    let mut i64_4: i64 = 3600i64;
    let mut i64_5: i64 = 1000000000i64;
    let mut str_1: &str = "month";
    let mut str_1_ref_0: &str = &mut str_1;
    let mut componentrange_1: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_1_ref_0, minimum: i64_5, maximum: i64_4, value: i64_3, conditional_range: bool_1};
    let mut bool_2: bool = false;
    let mut i64_6: i64 = 9223372036854775807i64;
    let mut i64_7: i64 = 604800i64;
    let mut i64_8: i64 = 60i64;
    let mut str_2: &str = "OffsetDateTime";
    let mut str_2_ref_0: &str = &mut str_2;
    let mut componentrange_2: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_2_ref_0, minimum: i64_8, maximum: i64_7, value: i64_6, conditional_range: bool_2};
    let mut bool_3: bool = false;
    let mut i64_9: i64 = 2440588i64;
    let mut i64_10: i64 = 60i64;
    let mut i64_11: i64 = 1000000000i64;
    let mut str_3: &str = "Monday";
    let mut str_3_ref_0: &str = &mut str_3;
    let mut componentrange_3: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_3_ref_0, minimum: i64_11, maximum: i64_10, value: i64_9, conditional_range: bool_3};
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_3);
    let mut error_1: error::Error = crate::error::Error::ComponentRange(componentrange_2);
    let mut error_2: error::Error = crate::error::Error::ComponentRange(componentrange_1);
    let mut error_3: error::Error = crate::error::Error::ComponentRange(componentrange_0);
//    panic!("From RustyUnit with love");
}
}