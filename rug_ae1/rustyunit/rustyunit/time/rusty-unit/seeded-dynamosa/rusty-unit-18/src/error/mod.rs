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
#[timeout(30000)]fn rusty_test_2457() {
//    rusty_monitor::set_test_id(2457);
    let mut bool_0: bool = true;
    let mut i64_0: i64 = 604800i64;
    let mut i64_1: i64 = 1000i64;
    let mut i64_2: i64 = 253402300799i64;
    let mut str_0: &str = "minutes";
    let mut str_0_ref_0: &str = &mut str_0;
    let mut componentrange_0: crate::error::component_range::ComponentRange = crate::error::component_range::ComponentRange {name: str_0_ref_0, minimum: i64_2, maximum: i64_1, value: i64_0, conditional_range: bool_0};
    let mut i8_0: i8 = 5i8;
    let mut i8_1: i8 = 24i8;
    let mut i8_2: i8 = 4i8;
    let mut utcoffset_0: crate::utc_offset::UtcOffset = crate::utc_offset::UtcOffset::__from_hms_unchecked(i8_2, i8_1, i8_0);
    let mut i32_0: i32 = 4i32;
    let mut i64_3: i64 = 604800i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::new(i64_3, i32_0);
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::abs(duration_0);
    let mut i32_1: i32 = 195i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_1};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_1);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::replace_date(offsetdatetime_0, date_1);
    let mut time_0: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u16_0: u16 = 0u16;
    let mut i32_2: i32 = 224i32;
    let mut date_2: crate::date::Date = crate::date::Date::__from_ordinal_date_unchecked(i32_2, u16_0);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_2, time_0);
    let mut primitivedatetime_1: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::offset_to_utc(primitivedatetime_0, utcoffset_0);
    let mut i128_0: i128 = 1000000000i128;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::nanoseconds_i128(i128_0);
    let mut i64_4: i64 = 1000000000i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_4);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_3: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_2, duration_3);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_3);
    let mut i32_3: i32 = 268i32;
    let mut date_3: crate::date::Date = crate::date::Date {value: i32_3};
    let mut primitivedatetime_2: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime {date: date_3, time: time_1};
    let mut month_0: month::Month = crate::month::Month::December;
    let mut month_1: month::Month = crate::month::Month::previous(month_0);
    let mut u16_1: u16 = crate::primitive_date_time::PrimitiveDateTime::ordinal(primitivedatetime_1);
    let mut error_0: error::Error = crate::error::Error::ComponentRange(componentrange_0);
    let mut error_0_ref_0: &error::Error = &mut error_0;
    let mut option_0: std::option::Option<&dyn std::error::Error> = std::error::Error::source(error_0_ref_0);
//    panic!("From RustyUnit with love");
}
}