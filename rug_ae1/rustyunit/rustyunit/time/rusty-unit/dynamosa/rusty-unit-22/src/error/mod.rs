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

#[no_coverage]
#[test]
#[should_panic]
#[timeout(3000)]
fn rusty_test_5237() {
    rusty_monitor::set_test_id(5237);
    let mut i64_0: i64 = -79i64;
    let mut duration_0: crate::duration::Duration = crate::duration::Duration::milliseconds(i64_0);
    let mut i8_0: i8 = -93i8;
    let mut i32_0: i32 = 108i32;
    let mut i64_1: i64 = 3i64;
    let mut duration_1: crate::duration::Duration = crate::duration::Duration::new(i64_1, i32_0);
    let mut i32_1: i32 = 8i32;
    let mut i64_2: i64 = -53i64;
    let mut duration_2: crate::duration::Duration = crate::duration::Duration::new(i64_2, i32_1);
    let mut i64_3: i64 = -11i64;
    let mut f32_0: f32 = -30.312741f32;
    let mut i64_4: i64 = 15i64;
    let mut duration_3: crate::duration::Duration = crate::duration::Duration::minutes(i64_4);
    let mut i32_2: i32 = 132i32;
    let mut i64_5: i64 = 37i64;
    let mut duration_4: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_5, i32_2);
    let mut i8_1: i8 = 20i8;
    let mut i8_2: i8 = -38i8;
    let mut i8_3: i8 = -19i8;
    let mut i16_0: i16 = -112i16;
    let mut duration_5: crate::duration::Duration = crate::duration::Duration::minutes(i64_3);
    let mut f64_0: f64 = -38.386991f64;
    let mut duration_6: crate::duration::Duration = crate::duration::Duration::seconds_f64(f64_0);
    let mut instant_0: crate::instant::Instant = crate::instant::Instant::now();
    let mut i8_4: i8 = 79i8;
    let mut i32_3: i32 = 188i32;
    let mut i64_6: i64 = 36i64;
    let mut duration_7: crate::duration::Duration = crate::duration::Duration::new_unchecked(i64_6, i32_3);
    let mut u32_0: u32 = 92u32;
    let mut u8_0: u8 = 43u8;
    let mut u8_1: u8 = 25u8;
    let mut u8_2: u8 = 87u8;
    let mut time_0: crate::time::Time = crate::time::Time::__from_hms_nanos_unchecked(u8_2, u8_1, u8_0, u32_0);
    let mut instant_1: crate::instant::Instant = crate::instant::Instant::now();
    let mut duration_8: crate::duration::Duration = crate::instant::Instant::elapsed(instant_1);
    let mut i32_4: i32 = -188i32;
    let mut date_0: crate::date::Date = crate::date::Date {value: i32_4};
    let mut date_1: crate::date::Date = crate::date::Date::saturating_sub(date_0, duration_8);
    let mut primitivedatetime_0: crate::primitive_date_time::PrimitiveDateTime = crate::primitive_date_time::PrimitiveDateTime::new(date_1, time_0);
    let mut i64_7: i64 = -60i64;
    let mut i32_5: i32 = 113i32;
    let mut i64_8: i64 = -35i64;
    let mut duration_9: crate::duration::Duration = crate::duration::Duration::minutes(i64_8);
    let mut duration_10: crate::duration::Duration = crate::duration::Duration::saturating_mul(duration_9, i32_5);
    let mut offsetdatetime_0: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut offsetdatetime_1: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::saturating_sub(offsetdatetime_0, duration_10);
    let mut time_1: crate::time::Time = crate::offset_date_time::OffsetDateTime::time(offsetdatetime_1);
    let mut u8_3: u8 = 82u8;
    let mut i64_9: i64 = 106i64;
    let mut duration_11: crate::duration::Duration = crate::duration::Duration::minutes(i64_9);
    let mut i32_6: i32 = -92i32;
    let mut date_2: crate::date::Date = crate::date::Date {value: i32_6};
    let mut date_3: crate::date::Date = crate::date::Date::saturating_add(date_2, duration_2);
    let mut option_0: std::option::Option<crate::date::Date> = crate::date::Date::previous_day(date_3);
    let mut offsetdatetime_2: crate::offset_date_time::OffsetDateTime = crate::offset_date_time::OffsetDateTime::now_utc();
    let mut u32_1: u32 = crate::time::Time::nanosecond(time_1);
    let mut duration_12: crate::duration::Duration = crate::duration::Duration::seconds(i64_7);
    let mut duration_13: crate::duration::Duration = crate::duration::Duration::abs(duration_12);
    let mut month_0: month::Month = crate::month::Month::December;
    let mut i16_1: i16 = crate::duration::Duration::subsec_milliseconds(duration_6);
    let mut result_0: std::result::Result<crate::utc_offset::UtcOffset, crate::error::component_range::ComponentRange> = crate::utc_offset::UtcOffset::from_hms(i8_3, i8_2, i8_1);
    let mut i64_10: i64 = crate::duration::Duration::whole_days(duration_13);
    panic!("From RustyUnit with love");
}
}