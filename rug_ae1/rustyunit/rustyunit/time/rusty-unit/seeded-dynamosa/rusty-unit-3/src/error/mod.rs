//! Various error types returned by methods in the time crate.

pub mod component_range;
pub mod conversion_range;
pub mod different_variant;
#[cfg(feature = "formatting")]
pub mod format;
#[cfg(feature = "local-offset")]
pub mod indeterminate_offset;
#[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
pub mod invalid_format_description;
#[cfg(feature = "parsing")]
pub mod parse;
#[cfg(feature = "parsing")]
pub mod parse_from_description;
#[cfg(feature = "parsing")]
pub mod try_from_parsed;

use core::fmt;

pub use component_range::ComponentRange;
pub use conversion_range::ConversionRange;
pub use different_variant::DifferentVariant;
#[cfg(feature = "formatting")]
pub use format::Format;
#[cfg(feature = "local-offset")]
pub use indeterminate_offset::IndeterminateOffset;
#[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
pub use invalid_format_description::InvalidFormatDescription;
#[cfg(feature = "parsing")]
pub use parse::Parse;
#[cfg(feature = "parsing")]
pub use parse_from_description::ParseFromDescription;
#[cfg(feature = "parsing")]
pub use try_from_parsed::TryFromParsed;

/// A unified error type for anything returned by a method in the time crate.
///
/// This can be used when you either don't know or don't care about the exact error returned.
/// `Result<_, time::Error>` (or its alias `time::Result<_>`) will work in these situations.
#[allow(missing_copy_implementations, variant_size_differences)]
#[allow(clippy::missing_docs_in_private_items)] // variants only
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    ConversionRange(ConversionRange),
    ComponentRange(ComponentRange),
    #[cfg(feature = "local-offset")]
    IndeterminateOffset(IndeterminateOffset),
    #[cfg(feature = "formatting")]
    Format(Format),
    #[cfg(feature = "parsing")]
    ParseFromDescription(ParseFromDescription),
    #[cfg(feature = "parsing")]
    #[non_exhaustive]
    UnexpectedTrailingCharacters,
    #[cfg(feature = "parsing")]
    TryFromParsed(TryFromParsed),
    #[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
    InvalidFormatDescription(InvalidFormatDescription),
    DifferentVariant(DifferentVariant),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConversionRange(e) => e.fmt(f),
            Self::ComponentRange(e) => e.fmt(f),
            #[cfg(feature = "local-offset")]
            Self::IndeterminateOffset(e) => e.fmt(f),
            #[cfg(feature = "formatting")]
            Self::Format(e) => e.fmt(f),
            #[cfg(feature = "parsing")]
            Self::ParseFromDescription(e) => e.fmt(f),
            #[cfg(feature = "parsing")]
            Self::UnexpectedTrailingCharacters => f.write_str("unexpected trailing characters"),
            #[cfg(feature = "parsing")]
            Self::TryFromParsed(e) => e.fmt(f),
            #[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
            Self::InvalidFormatDescription(e) => e.fmt(f),
            Self::DifferentVariant(e) => e.fmt(f),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ConversionRange(err) => Some(err),
            Self::ComponentRange(err) => Some(err),
            #[cfg(feature = "local-offset")]
            Self::IndeterminateOffset(err) => Some(err),
            #[cfg(feature = "formatting")]
            Self::Format(err) => Some(err),
            #[cfg(feature = "parsing")]
            Self::ParseFromDescription(err) => Some(err),
            #[cfg(feature = "parsing")]
            Self::UnexpectedTrailingCharacters => None,
            #[cfg(feature = "parsing")]
            Self::TryFromParsed(err) => Some(err),
            #[cfg(all(any(feature = "formatting", feature = "parsing"), feature = "alloc"))]
            Self::InvalidFormatDescription(err) => Some(err),
            Self::DifferentVariant(err) => Some(err),
        }
    }
}

#[cfg(test)]
mod rusty_tests {
	use crate::*;
	use std::error::Error;
//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_6814() {
//    rusty_monitor::set_test_id(6814);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_0: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut u32_0: u32 = 100000u32;
    let mut u8_0: u8 = 6u8;
    let mut u8_1: u8 = 9u8;
    let mut u8_2: u8 = 8u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut i32_0: i32 = 325i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_0};
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_0, time: time_0};
    let mut i64_0: i64 = -47i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::nanoseconds(i64_0);
    let mut f64_0: f64 = 4828193600913801216.000000f64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut u32_1: u32 = 10u32;
    let mut u8_3: u8 = 6u8;
    let mut u8_4: u8 = 29u8;
    let mut u8_5: u8 = 5u8;
    let mut time_1: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_5, u8_4, u8_3, u32_1);
    let mut i32_1: i32 = 359i32;
    let mut date_1: crate::date::Date = crate::date::Date::from_julian_day_unchecked(i32_1);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::date::Date::with_time(date_1, time_1);
    let mut option_0: std::option::Option<crate::duration::Duration> = crate::duration::Duration::checked_add(duration_2, duration_1);
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::saturating_sub(primitivedatetime_0, duration_0);
//    panic!("From RustyUnit with love");
}

//#[no_coverage]
#[test]
//#[should_panic]
#[timeout(30000)]fn rusty_test_5537() {
//    rusty_monitor::set_test_id(5537);
    let mut i32_0: i32 = 116i32;
    let mut u8_0: u8 = 12u8;
    let mut u8_1: u8 = 28u8;
    let mut u8_2: u8 = 1u8;
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 253402300799i64;
    let mut i64_1: i64 = 604800i64;
    let mut i64_2: i64 = -10i64;
    let mut str_0: &str = "week";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut u16_0: u16 = 59u16;
    let mut u8_3: u8 = 2u8;
    let mut u8_4: u8 = 8u8;
    let mut u8_5: u8 = 3u8;
    let mut u32_0: u32 = 10u32;
    let mut u8_6: u8 = 59u8;
    let mut u8_7: u8 = 4u8;
    let mut u8_8: u8 = 31u8;
    let mut i32_1: i32 = 139i32;
    let mut i64_3: i64 = 23i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_1);
    let mut i32_2: i32 = 9i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_2};
    let mut i32_3: i32 = 9999i32;
    let mut i64_4: i64 = 60i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_4, i32_3);
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::abs(duration_1);
    let mut i64_5: i64 = 3600i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::days(i64_5);
    let mut duration_4: std::time::Duration = crate::duration::Duration::abs_std(duration_3);
    let mut i64_6: i64 = 86400i64;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_6);
    let mut i128_0: i128 = 1000000i128;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_7: crate::duration::Duration = crate::instant::Instant::elapsed(instant_0);
    let mut duration_8: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_7, i32_0);
    let mut duration_9: std::time::Duration = crate::duration::Duration::abs_std(duration_8);
    let mut i64_7: i64 = 9223372036854775807i64;
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut i32_4: i32 = 1000000i32;
    let mut i64_8: i64 = 2147483647i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::new(i64_8, i32_4);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_0, duration_11);
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_1);
    let mut i32_5: i32 = 6i32;
    let mut i64_9: i64 = -294i64;
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_9, i32_5);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_12);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut f32_0: f32 = 1065353216.000000f32;
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::seconds_f32(f32_0);
    let mut duration_14: crate::duration::Duration = crate::duration::Duration::abs(duration_13);
    let mut offsetdatetime_4: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_5: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_add(offsetdatetime_4, duration_14);
    let mut utcoffset_1: crate::utc_offset::UtcOffset = crate::offset_date_time::OffsetDateTime::offset(offsetdatetime_5);
    let mut result_0: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms_milli(u8_5, u8_4, u8_3, u16_0);
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut result_1: std::result::Result<crate::time::Time, crate::error::component_range::ComponentRange> = crate::time::Time::from_hms(u8_2, u8_1, u8_0);
    let mut error_0_ref_0: &error::Error = &mut error_0;
    let mut option_0: std::option::Option<&dyn std::error::Error> = std::error::Error::source(error_0_ref_0);
//    panic!("From RustyUnit with love");
}
}